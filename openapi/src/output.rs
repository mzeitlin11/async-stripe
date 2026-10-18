use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Component, Path, PathBuf};

use anyhow::{Context, ensure};

use crate::crates::ALL_CRATES;

const MANAGED_MARKER: &str = ".async-stripe-codegen-output";

/// A filesystem writer rooted at one prepared staging directory.
pub struct OutputDirectory {
    root: PathBuf,
}

impl OutputDirectory {
    pub fn prepare(root: &Path) -> anyhow::Result<Self> {
        ensure!(!root.as_os_str().is_empty(), "output directory cannot be empty");
        ensure!(root != Path::new("/"), "refusing to clear the filesystem root");
        if root.exists() {
            let managed = root == Path::new("out") || root.join(MANAGED_MARKER).is_file();
            let empty = fs::read_dir(root)?.next().is_none();
            ensure!(
                managed || empty,
                "refusing to clear unmanaged non-empty output directory {}; choose an empty directory",
                root.display()
            );
            fs::remove_dir_all(root)
                .with_context(|| format!("could not clear output directory {}", root.display()))?;
        }
        fs::create_dir_all(root)?;
        fs::write(root.join(MANAGED_MARKER), "async-stripe codegen staging directory\n")?;
        Ok(Self { root: root.to_owned() })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn write(&self, content: impl AsRef<[u8]>, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let mut options = OpenOptions::new();
        options.truncate(true);
        self.write_or_append(content.as_ref(), path.as_ref(), options)
    }

    pub fn append(&self, content: impl AsRef<[u8]>, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let mut options = OpenOptions::new();
        options.append(true);
        self.write_or_append(content.as_ref(), path.as_ref(), options)
    }

    fn write_or_append(
        &self,
        content: &[u8],
        path: &Path,
        mut options: OpenOptions,
    ) -> anyhow::Result<()> {
        ensure!(
            !path.as_os_str().is_empty()
                && path.components().all(|component| matches!(component, Component::Normal(_))),
            "generated path must be relative and stay within the output directory: {}",
            path.display()
        );
        let destination = self.root.join(path);
        fs::create_dir_all(destination.parent().context("generated file has no parent")?)?;
        options
            .create(true)
            .write(true)
            .open(&destination)
            .with_context(|| format!("could not open {}", destination.display()))?
            .write_all(content)
            .with_context(|| format!("could not write {}", destination.display()))
    }
}

pub fn install_to_repository(staging_root: &Path, repo_root: &Path) -> anyhow::Result<()> {
    // Check the complete staging tree before replacing any repository directory.
    for krate in &*ALL_CRATES {
        let source = staging_root.join("crates").join(krate.crate_name());
        require_file(&source.join("src/mod.rs"))?;
        require_file(&source.join("Cargo.toml"))?;
    }
    for path in ["async-stripe-webhook/mod.rs", "tests/mod.rs", "crate_info.md"] {
        require_file(&staging_root.join(path))?;
    }

    for krate in &*ALL_CRATES {
        let source = staging_root.join("crates").join(krate.crate_name());
        let destination = repo_root.join("generated").join(krate.crate_name());
        replace_directory(&source.join("src"), &destination.join("src"))?;
        fs::copy(source.join("Cargo.toml"), destination.join("Cargo.toml"))?;
    }
    replace_directory(
        &staging_root.join("async-stripe-webhook"),
        &repo_root.join("async-stripe-webhook/src/generated"),
    )?;
    replace_directory(&staging_root.join("tests"), &repo_root.join("tests/tests/it/generated"))?;
    fs::copy(staging_root.join("crate_info.md"), repo_root.join("crate_info.md"))?;
    Ok(())
}

fn require_file(path: &Path) -> anyhow::Result<()> {
    ensure!(path.is_file(), "missing generated file {}", path.display());
    Ok(())
}

fn replace_directory(source: &Path, destination: &Path) -> anyhow::Result<()> {
    if destination.exists() {
        fs::remove_dir_all(destination)
            .with_context(|| format!("could not replace {}", destination.display()))?;
    }
    copy_directory(source, destination)
}

fn copy_directory(source: &Path, destination: &Path) -> anyhow::Result<()> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let destination = destination.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_directory(&entry.path(), &destination)?;
        } else {
            fs::copy(entry.path(), destination)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    static NEXT_DIRECTORY: AtomicUsize = AtomicUsize::new(0);

    struct TestDirectory(PathBuf);

    impl TestDirectory {
        fn new() -> Self {
            let unique = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
            let sequence = NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir()
                .join(format!("async-stripe-output-{}-{unique}-{sequence}", std::process::id()));
            fs::create_dir(&path).unwrap();
            Self(path)
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).unwrap();
        }
    }

    #[test]
    fn writes_within_custom_output_and_clears_stale_managed_files() {
        let root = TestDirectory::new();
        let output = OutputDirectory::prepare(&root.0).unwrap();
        output.write("first", "nested/file.rs").unwrap();
        output.append(" second", "nested/file.rs").unwrap();
        assert_eq!(fs::read_to_string(root.0.join("nested/file.rs")).unwrap(), "first second");
        OutputDirectory::prepare(&root.0).unwrap();
        assert!(!root.0.join("nested/file.rs").exists());
        assert!(root.0.join(MANAGED_MARKER).is_file());
    }

    #[test]
    fn refuses_to_clear_unmanaged_nonempty_directories() {
        let root = TestDirectory::new();
        fs::write(root.0.join("unrelated.txt"), "keep").unwrap();
        assert!(OutputDirectory::prepare(&root.0).is_err());
        assert_eq!(fs::read_to_string(root.0.join("unrelated.txt")).unwrap(), "keep");
        assert!(OutputDirectory::prepare(Path::new("")).is_err());
        assert!(OutputDirectory::prepare(Path::new("/")).is_err());
    }

    #[test]
    fn rejects_generated_paths_outside_the_staging_root() {
        let root = TestDirectory::new();
        let output = OutputDirectory::prepare(&root.0).unwrap();
        for path in ["", "../outside.rs", "nested/../../outside.rs", "/outside.rs"] {
            assert!(output.write("content", path).is_err(), "{path}");
            assert!(output.append("content", path).is_err(), "{path}");
        }
    }

    fn staging_fixture(root: &Path) {
        for krate in &*ALL_CRATES {
            let destination = root.join("crates").join(krate.crate_name());
            fs::create_dir_all(destination.join("src")).unwrap();
            fs::write(destination.join("src/mod.rs"), "// generated").unwrap();
            fs::write(destination.join("Cargo.toml"), "# generated manifest").unwrap();
        }
        for directory in ["async-stripe-webhook", "tests"] {
            fs::create_dir_all(root.join(directory)).unwrap();
            fs::write(root.join(directory).join("mod.rs"), "// generated").unwrap();
        }
        fs::write(root.join("crate_info.md"), "generated table").unwrap();
    }

    #[test]
    fn installation_replaces_owned_files_and_preserves_other_crate_files() {
        let root = TestDirectory::new();
        let staging = root.0.join("staging");
        let repository = root.0.join("repository");
        staging_fixture(&staging);
        let first = ALL_CRATES[0].crate_name();
        let crate_root = repository.join("generated").join(first);
        fs::create_dir_all(crate_root.join("src")).unwrap();
        fs::write(crate_root.join("src/stale.rs"), "// stale").unwrap();
        fs::write(crate_root.join("README.md"), "keep").unwrap();
        install_to_repository(&staging, &repository).unwrap();
        assert!(!crate_root.join("src/stale.rs").exists());
        assert_eq!(fs::read_to_string(crate_root.join("README.md")).unwrap(), "keep");
        assert_eq!(
            fs::read_to_string(crate_root.join("Cargo.toml")).unwrap(),
            "# generated manifest"
        );
        assert!(repository.join("async-stripe-webhook/src/generated/mod.rs").is_file());
        assert!(repository.join("tests/tests/it/generated/mod.rs").is_file());
        assert_eq!(
            fs::read_to_string(repository.join("crate_info.md")).unwrap(),
            "generated table"
        );
    }

    #[test]
    fn incomplete_staging_does_not_modify_the_repository() {
        let root = TestDirectory::new();
        let staging = root.0.join("staging");
        let repository = root.0.join("repository");
        staging_fixture(&staging);
        fs::remove_file(staging.join("tests/mod.rs")).unwrap();
        let first = repository.join("generated").join(ALL_CRATES[0].crate_name());
        fs::create_dir_all(first.join("src")).unwrap();
        fs::write(first.join("src/mod.rs"), "// existing").unwrap();
        assert!(install_to_repository(&staging, &repository).is_err());
        assert_eq!(fs::read_to_string(first.join("src/mod.rs")).unwrap(), "// existing");
    }
}

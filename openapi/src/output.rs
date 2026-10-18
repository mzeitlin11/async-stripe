use std::collections::BTreeMap;
use std::fs;
use std::path::{Component, Path, PathBuf};

use anyhow::{Context, bail, ensure};

use crate::crates::ALL_CRATES;

/// A complete, filesystem-independent set of files produced by one operation.
#[derive(Debug, Default)]
pub struct ArtifactSet {
    files: BTreeMap<PathBuf, Vec<u8>>,
}

impl ArtifactSet {
    #[cfg(test)]
    pub(crate) fn contents(&self, path: impl AsRef<Path>) -> Option<&[u8]> {
        self.files.get(path.as_ref()).map(Vec::as_slice)
    }

    pub fn write<C: AsRef<[u8]>, P: AsRef<Path>>(
        &mut self,
        path: P,
        content: C,
    ) -> anyhow::Result<()> {
        let path = normalized_relative_path(path.as_ref())?;
        if self.files.insert(path.clone(), content.as_ref().to_vec()).is_some() {
            bail!("generated file was written more than once: {}", path.display());
        }
        Ok(())
    }

    pub fn append<C: AsRef<[u8]>, P: AsRef<Path>>(
        &mut self,
        path: P,
        content: C,
    ) -> anyhow::Result<()> {
        let path = normalized_relative_path(path.as_ref())?;
        self.files.entry(path).or_default().extend_from_slice(content.as_ref());
        Ok(())
    }

    pub fn materialize(&self, root: &Path) -> anyhow::Result<()> {
        prepare_staging_root(root)?;
        fs::write(root.join(MANAGED_MARKER), "async-stripe codegen staging directory\n")?;
        self.write_files(root)
    }

    /// Materialize a file set without taking ownership of the entire root.
    ///
    /// The manifest records exactly which paths belonged to the preceding run,
    /// allowing stale owned files to be removed without touching unrelated files
    /// in a user-selected report directory.
    pub fn materialize_owned(&self, root: &Path, manifest_name: &str) -> anyhow::Result<()> {
        ensure!(!root.as_os_str().is_empty(), "output root cannot be empty");
        ensure!(root != Path::new("/"), "refusing to use the filesystem root as output");
        let manifest_path = normalized_relative_path(Path::new(manifest_name))?;
        ensure!(
            !self.files.contains_key(&manifest_path),
            "artifact set contains its ownership manifest: {}",
            manifest_path.display()
        );
        fs::create_dir_all(root)
            .with_context(|| format!("could not create output directory {}", root.display()))?;

        let previous_manifest = root.join(&manifest_path);
        if previous_manifest.exists() {
            let previous: Vec<String> = serde_json::from_slice(&fs::read(&previous_manifest)?)
                .with_context(|| {
                    format!("could not parse ownership manifest {}", previous_manifest.display())
                })?;
            for previous in previous {
                let previous = normalized_relative_path(Path::new(&previous))?;
                if self.files.contains_key(&previous) {
                    continue;
                }
                let destination = root.join(previous);
                if destination.symlink_metadata().is_ok() {
                    fs::remove_file(&destination).with_context(|| {
                        format!("could not remove stale artifact {}", destination.display())
                    })?;
                }
            }
        }

        self.write_files(root)?;
        let owned_paths =
            self.files.keys().map(|path| path.to_string_lossy().into_owned()).collect::<Vec<_>>();
        fs::write(previous_manifest, serde_json::to_vec_pretty(&owned_paths)?)?;
        Ok(())
    }

    fn write_files(&self, root: &Path) -> anyhow::Result<()> {
        for (path, content) in &self.files {
            let destination = root.join(path);
            let parent = destination
                .parent()
                .context("generated file unexpectedly had no parent directory")?;
            fs::create_dir_all(parent)
                .with_context(|| format!("could not create {}", parent.display()))?;
            fs::write(&destination, content)
                .with_context(|| format!("could not write {}", destination.display()))?;
        }
        Ok(())
    }
}

/// Generated bindings are one use of the general artifact-set abstraction.
pub type GeneratedWorkspace = ArtifactSet;

const MANAGED_MARKER: &str = ".async-stripe-codegen-output";

fn normalized_relative_path(path: &Path) -> anyhow::Result<PathBuf> {
    ensure!(!path.as_os_str().is_empty(), "generated path cannot be empty");
    ensure!(!path.is_absolute(), "generated path must be relative: {}", path.display());
    for component in path.components() {
        ensure!(
            matches!(component, Component::Normal(_)),
            "generated path must not escape its output root: {}",
            path.display()
        );
    }
    Ok(path.to_owned())
}

fn prepare_staging_root(root: &Path) -> anyhow::Result<()> {
    ensure!(!root.as_os_str().is_empty(), "output root cannot be empty");
    ensure!(root != Path::new("/"), "refusing to use the filesystem root as output");

    if root.exists() {
        // `out` is the CLI's fixed staging directory. Custom paths need a marker before we can
        // safely assume their contents belong to codegen.
        let is_managed = root == Path::new("out") || root.join(MANAGED_MARKER).is_file();
        let is_empty = fs::read_dir(root)?.next().is_none();
        ensure!(
            is_managed || is_empty,
            "refusing to clear unmanaged non-empty output directory {}; choose an empty directory",
            root.display()
        );
        fs::remove_dir_all(root)
            .with_context(|| format!("could not clear output directory {}", root.display()))?;
    }
    fs::create_dir_all(root)
        .with_context(|| format!("could not create output directory {}", root.display()))
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
    ensure!(source.is_dir(), "missing generated directory {}", source.display());
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
        let target = destination.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_directory(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
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
        let mut output = ArtifactSet::default();
        output.write("nested/file.rs", "first").unwrap();
        output.append("nested/file.rs", " second").unwrap();
        output.materialize(&root.0).unwrap();
        assert_eq!(fs::read_to_string(root.0.join("nested/file.rs")).unwrap(), "first second");
        ArtifactSet::default().materialize(&root.0).unwrap();
        assert!(!root.0.join("nested/file.rs").exists());
        assert!(root.0.join(MANAGED_MARKER).is_file());
    }

    #[test]
    fn failed_materialization_leaves_custom_output_managed_for_retry() {
        let root = TestDirectory::new();
        let mut output = ArtifactSet::default();
        output.write("conflict", "file blocks a directory").unwrap();
        output.write("conflict/nested.rs", "cannot write here").unwrap();

        assert!(output.materialize(&root.0).is_err());
        assert!(root.0.join("conflict").is_file());
        assert!(root.0.join(MANAGED_MARKER).is_file());

        ArtifactSet::default().materialize(&root.0).unwrap();
        assert!(!root.0.join("conflict").exists());
    }

    #[test]
    fn refuses_to_clear_unmanaged_nonempty_directories() {
        let root = TestDirectory::new();
        fs::write(root.0.join("unrelated.txt"), "keep").unwrap();
        assert!(ArtifactSet::default().materialize(&root.0).is_err());
        assert_eq!(fs::read_to_string(root.0.join("unrelated.txt")).unwrap(), "keep");
        assert!(ArtifactSet::default().materialize(Path::new("")).is_err());
        assert!(ArtifactSet::default().materialize(Path::new("/")).is_err());
    }

    #[test]
    fn rejects_generated_paths_outside_the_staging_root() {
        let mut output = ArtifactSet::default();
        for path in ["", "../outside.rs", "nested/../../outside.rs", "/outside.rs"] {
            assert!(output.write(path, "content").is_err(), "{path}");
            assert!(output.append(path, "content").is_err(), "{path}");
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

    #[test]
    fn owned_materialization_removes_only_stale_owned_files() {
        let directory = TestDirectory::new();
        let root = &directory.0;
        fs::write(root.join("unrelated.txt"), "keep me").unwrap();

        let mut first = ArtifactSet::default();
        first.write("summary.md", "first").unwrap();
        first.write("graphs/full.dot", "graph").unwrap();
        first.materialize_owned(root, ".manifest.json").unwrap();

        let mut second = ArtifactSet::default();
        second.write("summary.md", "second").unwrap();
        second.materialize_owned(root, ".manifest.json").unwrap();

        assert_eq!(fs::read_to_string(root.join("summary.md")).unwrap(), "second");
        assert!(!root.join("graphs/full.dot").exists());
        assert_eq!(fs::read_to_string(root.join("unrelated.txt")).unwrap(), "keep me");
    }
}

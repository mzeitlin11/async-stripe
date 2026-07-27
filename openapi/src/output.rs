use std::collections::{BTreeMap, BTreeSet};
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
        self.write_files(root)?;
        fs::write(root.join(MANAGED_MARKER), "async-stripe codegen staging directory\n")?;
        Ok(())
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

    pub fn file_count(&self) -> usize {
        self.files.len()
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
        let is_managed = root == Path::new("out") || root.join(MANAGED_MARKER).exists();
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

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct OutputStats {
    pub files: usize,
    pub lines: usize,
    pub bytes: usize,
}

impl OutputStats {
    fn add_file(&mut self, content: &[u8]) {
        self.files += 1;
        self.bytes += content.len();
        self.lines += content.iter().filter(|byte| **byte == b'\n').count();
        if !content.is_empty() && !content.ends_with(b"\n") {
            self.lines += 1;
        }
    }
}

#[derive(Debug, Clone)]
pub struct OutputInventory {
    pub total: OutputStats,
    pub by_crate: BTreeMap<String, OutputStats>,
}

impl OutputInventory {
    pub fn collect(root: &Path) -> anyhow::Result<Self> {
        let files = collect_files(root)?;
        let mut total = OutputStats::default();
        let mut by_crate = BTreeMap::<String, OutputStats>::new();

        for (path, content) in files {
            if path == Path::new(MANAGED_MARKER) {
                continue;
            }
            total.add_file(&content);
            if let Some(crate_name) = crate_name_for_staged_path(&path) {
                by_crate.entry(crate_name).or_default().add_file(&content);
            }
        }
        Ok(Self { total, by_crate })
    }
}

fn crate_name_for_staged_path(path: &Path) -> Option<String> {
    let mut components = path.components();
    match (components.next(), components.next()) {
        (Some(Component::Normal(first)), Some(Component::Normal(second))) if first == "crates" => {
            Some(second.to_string_lossy().into_owned())
        }
        (Some(Component::Normal(first)), _) if first == "async-stripe-webhook" => {
            Some("async-stripe-webhook".to_string())
        }
        (Some(Component::Normal(first)), _) if first == "tests" => Some("tests".to_string()),
        _ => None,
    }
}

/// Compare the staged output with every repository path owned by code generation.
pub fn compare_with_repository(staging_root: &Path, repo_root: &Path) -> anyhow::Result<()> {
    let expected = install_file_map(staging_root, repo_root)?;
    let actual = repository_generated_file_map(repo_root)?;
    if expected == actual {
        return Ok(());
    }

    let expected_paths = expected.keys().collect::<BTreeSet<_>>();
    let actual_paths = actual.keys().collect::<BTreeSet<_>>();
    let added = expected_paths.difference(&actual_paths).copied().collect::<Vec<_>>();
    let removed = actual_paths.difference(&expected_paths).copied().collect::<Vec<_>>();
    let changed = expected_paths
        .intersection(&actual_paths)
        .filter(|path| expected.get(**path) != actual.get(**path))
        .copied()
        .collect::<Vec<_>>();

    let mut message = format!(
        "generated output differs: {} changed, {} added, {} removed",
        changed.len(),
        added.len(),
        removed.len()
    );
    append_paths(&mut message, "changed", &changed);
    append_paths(&mut message, "added", &added);
    append_paths(&mut message, "removed", &removed);
    bail!(message)
}

fn append_paths(message: &mut String, label: &str, paths: &[&PathBuf]) {
    if paths.is_empty() {
        return;
    }
    message.push_str(&format!("\n{label}:"));
    for path in paths.iter().take(30) {
        message.push_str(&format!("\n  {}", path.display()));
    }
    if paths.len() > 30 {
        message.push_str(&format!("\n  ... and {} more", paths.len() - 30));
    }
}

pub fn install_to_repository(staging_root: &Path, repo_root: &Path) -> anyhow::Result<()> {
    for krate in &*ALL_CRATES {
        let staged_crate = staging_root.join("crates").join(krate.crate_name());
        let repository_crate = repo_root.join("generated").join(krate.crate_name());
        replace_directory(&staged_crate.join("src"), &repository_crate.join("src"))?;
        fs::copy(staged_crate.join("Cargo.toml"), repository_crate.join("Cargo.toml"))?;
    }
    replace_directory(
        &staging_root.join("async-stripe-webhook"),
        &repo_root.join("async-stripe-webhook/src/generated"),
    )?;
    replace_directory(&staging_root.join("tests"), &repo_root.join("tests/tests/it/generated"))?;
    fs::copy(staging_root.join("crate_info.md"), repo_root.join("crate_info.md"))?;
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

fn install_file_map(
    staging_root: &Path,
    repo_root: &Path,
) -> anyhow::Result<BTreeMap<PathBuf, Vec<u8>>> {
    let mut installed = BTreeMap::new();
    for (path, content) in collect_files(staging_root)? {
        if path == Path::new(MANAGED_MARKER) {
            continue;
        }
        let destination = if let Ok(rest) = path.strip_prefix("crates") {
            PathBuf::from("generated").join(rest)
        } else if let Ok(rest) = path.strip_prefix("async-stripe-webhook") {
            PathBuf::from("async-stripe-webhook/src/generated").join(rest)
        } else if let Ok(rest) = path.strip_prefix("tests") {
            PathBuf::from("tests/tests/it/generated").join(rest)
        } else if path == Path::new("crate_info.md") {
            PathBuf::from("crate_info.md")
        } else {
            continue;
        };
        ensure!(
            repo_root.join(&destination).starts_with(repo_root),
            "invalid install destination {}",
            destination.display()
        );
        installed.insert(destination, content);
    }
    Ok(installed)
}

fn repository_generated_file_map(repo_root: &Path) -> anyhow::Result<BTreeMap<PathBuf, Vec<u8>>> {
    let mut files = BTreeMap::new();
    for krate in &*ALL_CRATES {
        let crate_root = PathBuf::from("generated").join(krate.crate_name());
        collect_files_into(
            repo_root.join(&crate_root).join("src"),
            crate_root.join("src"),
            &mut files,
        )?;
        let cargo_path = crate_root.join("Cargo.toml");
        files.insert(cargo_path.clone(), fs::read(repo_root.join(cargo_path))?);
    }
    collect_files_into(
        repo_root.join("async-stripe-webhook/src/generated"),
        PathBuf::from("async-stripe-webhook/src/generated"),
        &mut files,
    )?;
    collect_files_into(
        repo_root.join("tests/tests/it/generated"),
        PathBuf::from("tests/tests/it/generated"),
        &mut files,
    )?;
    files.insert(PathBuf::from("crate_info.md"), fs::read(repo_root.join("crate_info.md"))?);
    Ok(files)
}

fn collect_files(root: &Path) -> anyhow::Result<BTreeMap<PathBuf, Vec<u8>>> {
    let mut files = BTreeMap::new();
    collect_files_into(root.to_owned(), PathBuf::new(), &mut files)?;
    Ok(files)
}

pub fn snapshot_files(root: &Path) -> anyhow::Result<BTreeMap<PathBuf, Vec<u8>>> {
    let mut files = collect_files(root)?;
    files.remove(Path::new(MANAGED_MARKER));
    Ok(files)
}

fn collect_files_into(
    root: PathBuf,
    prefix: PathBuf,
    files: &mut BTreeMap<PathBuf, Vec<u8>>,
) -> anyhow::Result<()> {
    if !root.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(&root)? {
        let entry = entry?;
        let path = entry.path();
        let relative = prefix.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            collect_files_into(path, relative, files)?;
        } else {
            files.insert(relative, fs::read(path)?);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::ArtifactSet;

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let unique = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        std::env::temp_dir().join(format!("async-stripe-codegen-{name}-{unique}"))
    }

    #[test]
    fn owned_materialization_removes_only_stale_owned_files() {
        let root = temp_dir("owned-artifacts");
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("unrelated.txt"), "keep me").unwrap();

        let mut first = ArtifactSet::default();
        first.write("summary.md", "first").unwrap();
        first.write("graphs/full.dot", "graph").unwrap();
        first.materialize_owned(&root, ".manifest.json").unwrap();

        let mut second = ArtifactSet::default();
        second.write("summary.md", "second").unwrap();
        second.materialize_owned(&root, ".manifest.json").unwrap();

        assert_eq!(fs::read_to_string(root.join("summary.md")).unwrap(), "second");
        assert!(!root.join("graphs/full.dot").exists());
        assert_eq!(fs::read_to_string(root.join("unrelated.txt")).unwrap(), "keep me");

        fs::remove_dir_all(root).unwrap();
    }
}

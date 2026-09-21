//! The few git questions the tool asks, through the `git` binary.

use std::path::{Path, PathBuf};
use std::process::Command;

use deepmind_patches::version::Change;

fn git(root: &Path, args: &[&str]) -> Result<Vec<u8>, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .map_err(|error| format!("git: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "git {}: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(output.stdout)
}

/// The file at a revision, or `None` when it did not exist there.
pub fn show(root: &Path, rev: &str, path: &Path) -> Result<Option<Vec<u8>>, String> {
    let spec = format!("{rev}:{}", path.to_string_lossy().replace('\\', "/"));
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["show", &spec])
        .output()
        .map_err(|error| format!("git: {error}"))?;
    Ok(output.status.success().then_some(output.stdout))
}

/// Paths changed between a base ref and the working tree, with how.
pub fn changes(root: &Path, base: &str) -> Result<Vec<(Change, PathBuf)>, String> {
    let merge_base = String::from_utf8_lossy(&git(root, &["merge-base", base, "HEAD"])?)
        .trim()
        .to_owned();
    let output = git(root, &["diff", "--name-status", "-M", &merge_base])?;
    let mut changes = Vec::new();
    for line in String::from_utf8_lossy(&output).lines() {
        let mut fields = line.split('\t');
        let (Some(status), Some(first)) = (fields.next(), fields.next()) else {
            continue;
        };
        let change = match status.chars().next() {
            Some('A') => Change::Added,
            Some('D') => Change::Deleted,
            Some('R') => Change::Renamed,
            _ => Change::Modified,
        };
        if change == Change::Renamed {
            changes.push((Change::Deleted, PathBuf::from(first)));
            if let Some(second) = fields.next() {
                changes.push((Change::Added, PathBuf::from(second)));
            }
        } else {
            changes.push((change, PathBuf::from(first)));
        }
    }
    Ok(changes)
}

/// Commits that touched a path, newest first.
pub fn log(root: &Path, path: &Path) -> Result<Vec<String>, String> {
    let spec = path.to_string_lossy().replace('\\', "/");
    let output = git(root, &["log", "--format=%H", "--follow", "--", &spec])?;
    Ok(String::from_utf8_lossy(&output)
        .lines()
        .map(str::to_owned)
        .collect())
}

/// The newest `v*` tag reachable from HEAD, if any.
pub fn latest_tag(root: &Path) -> Result<Option<String>, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["describe", "--tags", "--match", "v*", "--abbrev=0"])
        .output()
        .map_err(|error| format!("git: {error}"))?;
    if !output.status.success() {
        return Ok(None);
    }
    Ok(Some(
        String::from_utf8_lossy(&output.stdout).trim().to_owned(),
    ))
}

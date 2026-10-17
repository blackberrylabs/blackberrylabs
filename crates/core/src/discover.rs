//! Is a module installed? Its binaries are looked up on `PATH`, nothing else is read.

use std::path::{Path, PathBuf};

/// The directories of `PATH`, as the shell would search them.
pub fn path_dirs() -> Vec<PathBuf> {
    std::env::var_os("PATH").map(|p| std::env::split_paths(&p).collect()).unwrap_or_default()
}

/// The first `dir/name` that exists and is a regular file, in `dirs` order.
pub fn which(name: &str, dirs: &[PathBuf]) -> Option<PathBuf> {
    if name.contains('/') {
        let p = Path::new(name);
        return p.is_file().then(|| p.to_path_buf());
    }
    dirs.iter().map(|d| d.join(name)).find(|p| p.is_file())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_in_order_and_ignores_directories() {
        let tmp = tempfile::tempdir().unwrap();
        let a = tmp.path().join("a");
        let b = tmp.path().join("b");
        std::fs::create_dir_all(&a).unwrap();
        std::fs::create_dir_all(&b).unwrap();
        std::fs::create_dir_all(a.join("tool")).unwrap(); // a directory named like the tool
        std::fs::write(b.join("tool"), "").unwrap();
        assert_eq!(which("tool", &[a.clone(), b.clone()]), Some(b.join("tool")));
        assert_eq!(which("other", &[a, b]), None);
    }
}

use std::path::{Path, PathBuf};

use walkdir::WalkDir;

pub fn find_scans(path: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    WalkDir::new(path)
        .into_iter()
        .filter_map(|ok| ok.ok())
        .filter(|e| e.file_type().is_file())
        .for_each(|e| {
            let ext = match e.path().extension() {
                Some(ext) => ext,
                None => return,
            };

            if ext == "raw" {
                paths.push(e.path().to_path_buf());
            }
        });
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_scans() {
        let path = Path::new("tests");
        let paths = find_scans(path);
        assert_eq!(paths.len(), 3);
    }
}

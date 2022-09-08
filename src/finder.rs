use std::path::{Path, PathBuf};

use walkdir::WalkDir;

pub struct Finder<'a> {
    pub input: &'a Path,
}

impl<'a> Finder<'a> {
    pub fn new(input: &'a Path) -> Self {
        Self { input }
    }

    pub fn find_scans(&self) -> Vec<PathBuf> {
        let mut paths = Vec::new();
        WalkDir::new(self.input)
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

    pub fn list_scans(&self) {
        self.find_scans()
            .iter()
            .for_each(|path| println!("{}", path.display()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_scans() {
        let path = Path::new("tests");
        let paths = Finder::new(path).find_scans();
        assert_eq!(paths.len(), 3);
    }
}

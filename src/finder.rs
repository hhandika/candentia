use std::{
    fs::OpenOptions,
    io::{BufWriter, Result, Write},
    path::{Path, PathBuf},
};

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
        let scans = self.find_scans();
        log::info!("Found {} scans", scans.len());
        self.write_metadata(&scans)
            .expect("Failed writing metadata");
    }

    fn write_metadata(&self, scans: &[PathBuf]) -> Result<()> {
        let output = self.input.join("scans.csv");
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(output)?;
        let mut writer = BufWriter::new(file);
        write!(writer, "path,size,created,accessed")?;
        scans.iter().for_each(|path| {
            let metadata = path.metadata().expect("Failed reading metadata");
            let size = metadata.len();
            let created = metadata.created().expect("Failed reading creation time");
            let accessed = metadata.accessed().expect("Failed reading access time");
            writeln!(
                writer,
                "{},{},{:?},{:?}",
                path.display(),
                size,
                created,
                accessed
            )
            .expect("Failed writing metadata");
        });
        Ok(())
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

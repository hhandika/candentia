use std::fs;
use std::path::{Path, PathBuf};

use regex::Regex;

const VOUCHER_NAME: &str = r"^\D+(_{0,1}\d{3,10})";

pub struct Organizer<'a> {
    pub scans: &'a [PathBuf],
    pub output: &'a Path,
}

impl<'a> Organizer<'a> {
    pub fn new(scans: &'a [PathBuf], output: &'a Path) -> Self {
        Self { scans, output }
    }

    pub fn organize(&self) {
        log::info!("{:18}: {}", "File counts", self.scans.len());
        self.scans.iter().for_each(|scan| {
            let scan_name = scan
                .file_name()
                .expect("Scan has no name")
                .to_string_lossy()
                .to_string();
            let voucher_name = match self.capture_voucher_name(&scan_name) {
                Some(name) => name,
                None => {
                    eprintln!("Could not capture voucher name from {}", scan_name);
                    return;
                }
            };

            let output_path = self.output.join(voucher_name);
            if output_path.is_dir() {
                log::info!("{} already exists. Skipping!", output_path.display());
                return;
            } else {
                fs::create_dir_all(&output_path).expect("Could not create voucher directory");
                fs::rename(scan, output_path.join(&scan_name)).expect("Could not move scan");
            }
        });
        log::info!("Done");
    }

    fn capture_voucher_name(&self, name: &str) -> Option<String> {
        let re = Regex::new(VOUCHER_NAME).expect("Invalid regex");
        re.captures(name).map(|cap| cap[0].to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capture_voucher_name() {
        let organizer = Organizer::new(&[], &Path::new(""));
        let name = "Uromys_spce_MUSEUM_12345_2080_1066_960_2027_0.0119013mm_16be_un.raw";
        let name_wo_underscore =
            "Uromys_spce_MUSEUM12345_2080_1066_960_2027_0.0119013mm_16be_un.raw";
        let voucher_name = organizer.capture_voucher_name(name);
        let voucher_name_wo_underscore = organizer.capture_voucher_name(name_wo_underscore);
        assert_eq!(voucher_name, Some("Uromys_spce_MUSEUM_12345".to_string()));
        assert_eq!(
            voucher_name_wo_underscore,
            Some("Uromys_spce_MUSEUM12345".to_string())
        );
    }
}

use std::path::Path;

mod finder;
mod organizer;

fn main() {
    let path = Path::new(".");
    let file_paths = finder::find_scans(path);
    file_paths
        .iter()
        .for_each(|path| println!("{}", path.display()));
    organizer::Organizer::new(&file_paths, path).organize();
}

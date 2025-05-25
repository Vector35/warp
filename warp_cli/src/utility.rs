use std::path::Path;
use warp::WarpFile;

/// Construct an owned file from a path, this means the file owns the buffer, and we have no problems passing it to-from.
pub fn open_file<'fbb>(path: &Path) -> WarpFile<'fbb> {
    let file = std::fs::read(path).expect("Unable to read file");
    let file = WarpFile::from_owned_bytes(file).expect("Invalid WARP file");
    file
}

use crate::operation::OperationHandler;
use crate::utility::open_file;
use clap::Parser;
use std::path::{Path, PathBuf};
use warp::chunk::{Chunk, CompressionType};
use warp::{WarpFile, WarpFileHeader};

/// Merge all other files into the provided path.
#[derive(Clone, Debug, Parser)]
pub struct MergeOp {
    /// Files to be merged into the given path.
    #[arg(index = 1)]
    unmerged_files: Vec<PathBuf>,
}

impl OperationHandler for MergeOp {
    fn run(&self, path: &Path) {
        let files: Vec<_> = self
            .unmerged_files
            .iter()
            .map(PathBuf::as_path)
            .map(open_file)
            .collect();
        let chunks: Vec<Chunk> = files.iter().flat_map(|f| f.chunks.clone()).collect();
        let merged_chunks = Chunk::merge(&chunks, CompressionType::Zstd);
        let merged_file = WarpFile::new(WarpFileHeader::new(), merged_chunks);
        std::fs::write(path, merged_file.to_bytes()).expect("Failed to write file");
    }
}

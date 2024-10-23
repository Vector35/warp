use flatbuffers_build::BuilderOptions;
use std::path::PathBuf;

#[cfg(feature = "dont_build")]
pub fn main() {}

#[cfg(not(feature = "dont_build"))]
pub fn main() {
    // Remove leftover symlink dir.
    let _ = std::fs::remove_dir_all("src/gen_flatbuffers");
    let workspace_dir: PathBuf = env!("CARGO_WORKSPACE_DIR").into();
    BuilderOptions::new_with_files([
        workspace_dir.join("type.fbs"),
        workspace_dir.join("symbol.fbs"),
        workspace_dir.join("signature.fbs"),
    ])
        .set_output_path("src/gen_flatbuffers")
        .compile()
        .expect("flatbuffer compilation failed");
}

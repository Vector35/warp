#[cfg(not(feature = "gen_flatbuffers"))]
pub fn main() {}

#[cfg(feature = "gen_flatbuffers")]
pub fn main() {
    use flatbuffers_build::BuilderOptions;
    use std::path::PathBuf;

    // Remove leftover symlink dir.
    let _ = std::fs::remove_dir_all("rust/gen_flatbuffers");
    let workspace_dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
    BuilderOptions::new_with_files([
        workspace_dir.join("type.fbs"),
        workspace_dir.join("symbol.fbs"),
        workspace_dir.join("signature.fbs"),
    ])
    .set_output_path("rust/gen_flatbuffers")
    .compile()
    .expect("flatbuffer compilation failed");
}

pub fn main() {
    use flatbuffers_build::BuilderOptions;
    use std::path::PathBuf;

    // Remove leftover symlink dir.
    let _ = std::fs::remove_dir_all("src/gen_flatbuffers");
    let workspace_dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
    BuilderOptions::new_with_files([
        workspace_dir.join("type.fbs"),
        workspace_dir.join("symbol.fbs"),
        workspace_dir.join("signature.fbs"),
    ])
    .set_symlink_directory("rust/gen_flatbuffers")
    .compile()
    .expect("flatbuffer compilation failed");
}

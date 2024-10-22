use std::path::PathBuf;

fn main() {
    let link_path =
        std::env::var_os("DEP_BINARYNINJACORE_PATH").expect("DEP_BINARYNINJACORE_PATH specified");

    println!("cargo::rustc-link-lib=dylib=binaryninjacore");
    println!("cargo::rustc-link-search={}", link_path.to_str().unwrap());

    #[cfg(not(target_os = "windows"))]
    {
        println!(
            "cargo::rustc-link-arg=-Wl,-rpath,{0},-L{0}",
            link_path.to_string_lossy()
        );
    }

    // Generate test binaries.
    let test_dir: PathBuf = "fixtures/".into();
    let object_files = cc::Build::new()
        .file(test_dir.join("library.c"))
        .compile_intermediates();

    // We need to have the object file paths passed as keys?
    println!(
        "cargo:rustc-env=TEST_BIN_LIBRARY_OBJ={}",
        object_files[0].to_string_lossy()
    );

    // TODO: How do we get this lib file?
    cc::Build::new()
        .file(test_dir.join("simple.c"))
        .compile("simple");
}

use fs_extra::dir::{CopyOptions, copy};
use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let resources_dir = PathBuf::from(&manifest_dir).join("resources");

    println!("cargo:rerun-if-changed={}", resources_dir.display());

    let out_dir = env::var("OUT_DIR").unwrap();
    let mut target_dir = PathBuf::from(out_dir);
    target_dir.pop();
    target_dir.pop();
    target_dir.pop(); // target_dir is now "target/debug" or "target/release"

    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;

    if let Err(e) = copy(&resources_dir, &target_dir, &options) {
        panic!("Error copying static files: {}", e);
    }

    // WASM specific: Preload resources for Emscripten
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "emscripten" {
        // Find the relative path from the target directory to the resources
        // For emscripten, we need to instruct the linker to preload the resources folder.
        // We do this by passing `--preload-file` to emcc. We use the path relative
        // to where the build is happening or an absolute path.
        // Since we copied resources to target_dir, we can use that absolute path,
        // but Emscripten usually maps paths.
        // The easiest way is to use the absolute path of the source resources directory 
        // mapped to the root of the virtual filesystem.
        println!("cargo:rustc-link-arg=--preload-file={}@/resources", resources_dir.display());
    }
}

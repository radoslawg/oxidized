use fs_extra::dir::{CopyOptions, copy};
use std::env;
use std::path::PathBuf;

fn main() {
    // Monitor directory 'resources'
    println!("cargo:rerun-if-changed=resources");

    let out_dir = env::var("OUT_DIR").unwrap();

    let mut target_dir = PathBuf::from(out_dir);
    target_dir.pop();
    target_dir.pop();
    target_dir.pop();

    // target_dir is "target/debug"

    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;

    // Wykonanie kopiowania
    let result = copy("resources", &target_dir, &options);

    if let Err(e) = result {
        panic!("Error copying static files: {}", e);
    }
}

use std::env;

fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rustc-link-search={}/arcsoft/lib/linux_x64", project_dir); // the "-L" flag
    println!("cargo:rustc-link-lib=arcsoft_face_engine"); // the "-l" flag
    println!("cargo:rustc-link-lib=arcsoft_face"); // the "-l" flag
}

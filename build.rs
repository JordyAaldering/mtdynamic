use std::env;

fn main() {
    let lib_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap();
    let path = format!("target/{}/mtdynamic.h", profile);

    cbindgen::Builder::new()
        .with_crate(lib_dir)
        .with_language(cbindgen::Language::C)
        .with_include_guard("_MTD_MTDYNAMIC_")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(path);
}

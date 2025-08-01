use std::env;

fn main() {
    let lib_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap();
    let header_path = format!("../target/{}/letterbox.h", profile);

    cbindgen::Builder::new()
        .with_crate(lib_dir)
        .with_tab_width(4)
        .with_no_includes()
        .with_sys_include("stdint.h")
        .with_include_guard("_MTD_LETTERBOX_H_")
        .with_after_include("\n#define MTD_LETTERBOX_PATH \"/tmp/mtd_letterbox\"")
        .include_item("Demand")
        .include_item("Request")
        .include_item("Sample")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(header_path);
}

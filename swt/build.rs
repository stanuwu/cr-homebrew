extern crate cbindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("include/libswt.h");

    let devkitpro = "C:\\devkitPro";

    println!("cargo:rustc-link-lib=static=nx");
    println!("cargo:rustc-link-search=native={}/libnx/lib", devkitpro);

    bindgen::Builder::default()
        .trust_clang_mangling(false)
        .use_core()
        .ctypes_prefix("lang_items")
        .header("wrapper.h")
        .clang_arg(format!("-I{}/libnx/include", devkitpro))
        .clang_arg(format!("-I{}/devkitA64/aarch64-none-elf/include", devkitpro))
        .bitfield_enum("HidMouseButton")
        .bitfield_enum("HidKeyboardModifier")
        .rustified_enum("HidKeyboardScancode")
        .bitfield_enum("HidControllerType")
        .rustified_enum("HidControllerLayoutType")
        .bitfield_enum("HidControllerColorDescription")
        .bitfield_enum("HidControllerKeys")
        .rustified_enum("HidControllerJoystick")
        .bitfield_enum("HidControllerConnectionState")
        .rustified_enum("HidControllerID")
        .generate_inline_functions(true)
        .blocklist_type("u8")
        .blocklist_type("u16")
        .blocklist_type("u32")
        .blocklist_type("u64")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"));
}
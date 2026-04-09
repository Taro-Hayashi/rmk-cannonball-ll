use std::io::Read;
use std::path::PathBuf;
use std::{env, fs};

use const_gen::*;
use xz2::read::XzEncoder;

fn main() {
    generate_vial_config();
    copy_linker_script();
}

fn copy_linker_script() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(out.join("memory.x"), include_bytes!("memory.x")).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rustc-link-arg=--nmagic");
    println!("cargo:rustc-link-arg=-Tlink.x");
    println!("cargo:rustc-link-arg=-Tdefmt.x");
}

fn generate_vial_config() {
    println!("cargo:rerun-if-changed=vial.json");

    let content = fs::read_to_string("vial.json").expect("Cannot read vial.json");
    let vial_cfg = json::stringify(json::parse(&content).unwrap());

    let mut keyboard_def_compressed = Vec::new();
    XzEncoder::new(vial_cfg.as_bytes(), 6)
        .read_to_end(&mut keyboard_def_compressed)
        .unwrap();

    let keyboard_id: Vec<u8> = vec![0xC4, 0xBB, 0x09, 0x07, 0x88, 0x84, 0xAA, 0x11];
    let const_declarations = [
        const_declaration!(pub VIAL_KEYBOARD_DEF = keyboard_def_compressed),
        const_declaration!(pub VIAL_KEYBOARD_ID = keyboard_id),
    ]
    .map(|s| "#[allow(clippy::redundant_static_lifetimes)]\n".to_owned() + s.as_str())
    .join("\n");

    let out_file = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("config_generated.rs");
    fs::write(out_file, const_declarations).unwrap();
}

use std::fs;
use std::path::PathBuf;

use crate::scripts::build::parse::parse_value;

const BUILD_GEN_DIR: &str = "build_gen";

const LINKER_SYMBOLS: &[&str] =
    &["KERNEL_VMA_BASE", "KERNEL_LMA_BASE", "PHYSICAL_BASE"];

fn linker_script_path(target_arch: &str) -> PathBuf {
    PathBuf::from(format!("linker/{}.ld", target_arch))
}

fn set_linker_script(linker_script: &PathBuf) {
    println!("cargo:rerun-if-changed={}", linker_script.display());
    println!("cargo:rustc-link-arg=-T{}", linker_script.display());
    println!(
        "cargo:info=Task [Linker]: Set to {}",
        linker_script.display()
    );
}

fn generate_const(linker_script: &PathBuf) -> PathBuf {
    let content = fs::read_to_string(linker_script)
        .expect("Failed to read linker script");

    let constants: Vec<(&str, usize)> = LINKER_SYMBOLS
        .iter()
        .map(|&name| (name, parse_value(&content, name)))
        .collect();

    let gen_dir = PathBuf::from("src").join(BUILD_GEN_DIR);
    fs::create_dir_all(&gen_dir).unwrap();

    let header = linker_script.display();

    // Generate Rust file
    let mut rs =
        format!("//! DO NOT EDIT! Generated from `{header}`.\n\n");
    for &(name, value) in &constants {
        rs.push_str(&format!(
            "pub const {name}: usize = {value:#X};\n"
        ));
    }
    let rs_path = gen_dir.join("linker_const.rs");
    fs::write(&rs_path, rs).unwrap();

    // Generate assembly header file
    let mut h = format!(
        "/* DO NOT EDIT! Generated from `{header}` */\n\n\
         #ifndef _LINKER_CONST_H_\n\
         #define _LINKER_CONST_H_\n\n"
    );
    for &(name, value) in &constants {
        h.push_str(&format!("#define {name:<16}{value:#X}\n"));
    }
    h.push_str("\n#endif // _LINKER_CONST_H_\n");
    let h_path = gen_dir.join("linker_const.h");
    fs::write(&h_path, h).unwrap();

    println!(
        "cargo:info=Task [ConstGen]: Generated constants into {:?}",
        &gen_dir
    );
    gen_dir
}

/// Linker setup task:
/// 1. Set the linker script;
/// 2. Generate constants from the linker script for use in Rust and
///    assembly.
///
/// rerurn the path to the generated include files for
/// subsequent tasks to use(asm compile).
pub fn setup(target_arch: &str) -> PathBuf {
    let linker_script = linker_script_path(target_arch);
    set_linker_script(&linker_script);
    generate_const(&linker_script)
}

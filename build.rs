mod scripts;

use scripts::build;

fn main() {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH")
        .expect("CARGO_CFG_TARGET_ARCH not set");

    println!(
        "cargo:info=--- Running Build Tasks for {} ---",
        &target_arch
    );

    // linker
    let asm_inc_path = build::linker::setup(&target_arch);

    build::asm::compile(&target_arch, &asm_inc_path);

    println!("cargo:info=--- All Build Tasks Completed ---");
}

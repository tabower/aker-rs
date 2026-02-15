use std::fs;
use std::path::Path;

/// Recursively collect all assembly files under the given directory.
/// Matches extensions: `.S`, `.s`, `.asm`
fn collect_asm_files(dir: &Path) -> Vec<String> {
    let mut files = Vec::new();

    let entries = fs::read_dir(dir).unwrap_or_else(|e| {
        panic!("Failed to read directory '{}': {}", dir.display(), e);
    });

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            files.extend(collect_asm_files(&path));
        } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            match ext {
                "S" | "s" | "asm" => {
                    files.push(path.to_string_lossy().into_owned());
                }
                _ => {}
            }
        }
    }

    files.sort();
    files
}

/// Compile the corresponding assembly files according to the architecture
/// and apply architecture-specific compiler flags.
pub fn compile(target_arch: &str, include_path: &Path) {
    let mut build = cc::Build::new();

    let arch_dir = Path::new("src/arch").join(target_arch);

    match target_arch {
        "riscv64" => {
            build.flag("-march=rv64gc_zihintpause");
            build.flag("-mabi=lp64d");
        }
        _ => {}
    };

    let asm_files = collect_asm_files(&arch_dir);

    if asm_files.is_empty() {
        println!(
            "cargo:warning=Task [AsmBuild]: No assembly files found under '{}' for architecture '{}'.",
            arch_dir.display(),
            target_arch
        );
        return;
    }

    build.include(include_path);
    build.files(asm_files.iter());
    build.compile("asm_symbols");

    for file in &asm_files {
        println!("cargo:rerun-if-changed={}", file);
    }

    println!(
        "cargo:info=Task [AsmBuild]: Compiled {} assembly file(s) for {}.",
        asm_files.len(),
        target_arch
    );
}

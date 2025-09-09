#!/usr/bin/env rust-script

use std::env;
use std::process::{Command, exit};
use std::path::Path;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: ti83-runner <elf-path>");
        exit(1);
    }
    
    let elf_path = &args[args.len() - 1];
    let elf_name = Path::new(elf_path).file_stem().unwrap().to_str().unwrap();
    
    // Variables de configuration
    let cedev = "./CEdev";
    let executable_name = "DEMO";
    
    println!("=== Build TI-83 Premium CE ===");
    println!("Source ELF: {}", elf_path);
    println!("Nom de l'exécutable: {}", executable_name);
    
    // Créer les dossiers
    create_dirs();
    
    // Trouver le fichier LLVM IR le plus récent
    let ll_path = find_latest_llvm_ir_file(elf_name);
    if ll_path.is_none() {
        eprintln!("❌ Fichier LLVM IR non trouvé pour {}", elf_name);
        exit(1);
    }
    let ll_path = ll_path.unwrap()[0].clone();
    
    // Copier et nettoyer le fichier LLVM IR
    if !copy_and_clean_llvm_ir(&ll_path, elf_name) {
        exit(1);
    }
    
    // Transpiler LLVM IR vers assembleur
    if !transpile_llvm_ir(cedev, elf_name) {
        exit(1);
    }
    
    // Compiler l'assembleur
    if !compile_asm(cedev, elf_name) {
        exit(1);
    }
    
    // Créer le fichier 8xp
    if !make_8xp(cedev, elf_name, executable_name) {
        exit(1);
    }
    
    println!("✅ Build terminé ! Le fichier {}.8xp est disponible dans bin/", executable_name);
}

fn create_dirs() {
    let _ = std::fs::create_dir_all("bin");
    let _ = std::fs::create_dir_all("incremental");
}

// get the file with before -<elf_name>
fn extract_file_stem(file_path: &str) -> Option<String> {
    Path::new(file_path).file_stem().and_then(|s| s.to_str()).map(|s| s.to_string()).map(|s| s.split('-').next().unwrap_or("").to_string())
}

fn find_latest_llvm_ir_file(elf_name: &str) -> Option<Vec<String>> {
    let deps_dir = "target/ez80-tice-none/release/deps";
    // Trouver tous les fichiers LLVM IR du projet principal
    let mut main_files = Vec::new();
    if let Ok(entries) = fs::read_dir(deps_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                if filename.ends_with(".ll") {
                    if let Ok(metadata) = entry.metadata() {
                        if let Ok(modified) = metadata.modified() {
                            let name = extract_file_stem(&path.to_string_lossy()).unwrap_or("unknow".to_string());
                            main_files.push((path.to_string_lossy().to_string(), name, modified));
                        }
                    }
                }
            }
        }
    }

    let mut libs = std::collections::HashSet::new();
    main_files.sort_by(|a, b| b.2.cmp(&a.2).then_with(|| b.1.cmp(&a.1)));
    let mut files = Vec::new();

    for (path, name, _) in &main_files {
        if !libs.contains(name) {
            libs.insert(name.clone());
            files.push(path.clone());
            println!("📄 Fichier LLVM IR trouvé: {} ({})", path, name);
        }
    }

    Some(files)
}

fn copy_and_clean_llvm_ir(src_path: &str, elf_name: &str) -> bool {
    println!("🔧 Copie et nettoyage du fichier LLVM IR...");
    let dest_path = format!("./incremental/{}.ll", elf_name);
    
    // Lire le fichier source
    let content = fs::read_to_string(src_path).unwrap_or_else(|_| {
        eprintln!("❌ Erreur lecture {}", src_path);
        exit(1);
    });
    
    // Nettoyer les attributs LLVM modernes incompatibles avec ez80-clang
    let cleaned_content = content
        .replace("wasm32-unknown-unknown", "ez80")
        .replace("captures(none)", "nocapture");
    
    // Écrire le fichier nettoyé
    fs::write(&dest_path, cleaned_content).unwrap_or_else(|_| {
        eprintln!("❌ Erreur écriture {}", dest_path);
        exit(1);
    });
    
    println!("   {} -> {} (nettoyé)", src_path, dest_path);
    true
}

fn run_command(mut cmd: Command) -> bool {
    match cmd.status() {
        Ok(status) if status.success() => true,
        Ok(status) => {
            eprintln!("Commande échouée avec le code: {}", status.code().unwrap_or(-1));
            false
        }
        Err(e) => {
            eprintln!("Erreur lors de l'exécution de la commande: {}", e);
            false
        }
    }
}

fn transpile_llvm_ir(cedev: &str, elf_name: &str) -> bool {
    println!("🔧 Transpilation LLVM IR vers assembleur...");
    let mut cmd = Command::new(&format!("{}/bin/ez80-clang", cedev));
    cmd.args(&[
        "-S",
        &format!("./incremental/{}.ll", elf_name),
        "-o", &format!("./incremental/{}.s", elf_name)
    ]);
    run_command(cmd)
}

fn compile_asm(cedev: &str, elf_name: &str) -> bool {
    println!("🔧 Compilation assembleur...");
    let mut cmd = Command::new(&format!("{}/bin/fasmg", cedev));
    cmd.args(&[
        "-v1",
        &format!("{}/meta/ld.alm", cedev),
        "-i", "DEBUG := 1",
        "-i", "HAS_PRINTF := 1",
        "-i", "HAS_LIBC := 1",
        "-i", "HAS_LIBCXX := 0",
        "-i", "PREFER_OS_CRT := 0",
        "-i", "PREFER_OS_LIBC := 1",
        "-i", "ALLOCATOR_STANDARD := 1",
        "-i", "__TICE__ := 1",
        "-i", &format!("include \"{}/meta/linker_script\"", cedev),
        "-i", "range .bss $D052C6 : $D13FD8",
        "-i", "provide __stack = $D1A87E",
        "-i", "locate .header at $D1A87F",
        "-i", "map",
        "-i", &format!("source \"{}/lib/crt/crt0.src\", \"./incremental/{}.s\"", cedev, elf_name),
        "-i", &format!("library \"{}/lib/libload/fatdrvce.lib\", \"{}/lib/libload/fileioc.lib\", \"{}/lib/libload/fontlibc.lib\", \"{}/lib/libload/graphx.lib\", \"{}/lib/libload/keypadc.lib\", \"{}/lib/libload/msddrvce.lib\", \"{}/lib/libload/srldrvce.lib\", \"{}/lib/libload/usbdrvce.lib\"", cedev, cedev, cedev, cedev, cedev, cedev, cedev, cedev),
        &format!("incremental/{}.bin", elf_name)
    ]);
    run_command(cmd)
}

fn make_8xp(cedev: &str, elf_name: &str, executable_name: &str) -> bool {
    println!("🔧 Création du fichier 8xp...");
    let mut cmd = Command::new(&format!("{}/bin/convbin", cedev));
    cmd.args(&[
        "--oformat", "8xp",
        "--uppercase",
        "--name", executable_name,
        "--input", &format!("incremental/{}.bin", elf_name),
        "--output", &format!("bin/{}.8xp", executable_name)
    ]);
    run_command(cmd)
}

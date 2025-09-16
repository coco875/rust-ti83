use std::path::Path;

fn create_dirs() {
    let _ = std::fs::create_dir_all("incremental");
}

fn patch_file(file: &str) {
    let content = std::fs::read_to_string(file).expect("Erreur lors de la lecture du fichier");
    let patched_content = content
        .replace("thumbv6m-none", "ez80");

    // fix call convertion (ti flags)
    let patched_content = patched_content
        // sys/power.h
        .replace("void @os_DisableAPD", "cc102 void @os_DisableAPD")
        .replace("void @os_EnableAPD", "cc102 void @os_EnableAPD")
        .replace("i8 @boot_GetBatteryStatus", "cc102 i8 @boot_GetBatteryStatus")
        // ti/real.h
        .replace("%real_t @os_Int24ToReal", "cc102 %real_t @os_Int24ToReal")
        // ti/screen.h
        .replace("void @os_NewLine", "cc102 void @os_NewLine")
        .replace("void @os_MoveUp", "cc102 void @os_MoveUp")
        .replace("void @os_MoveDown", "cc102 void @os_MoveDown")
        .replace("void @os_HomeUp", "cc102 void @os_HomeUp")
        .replace("void @os_ClrLCDFull", "cc102 void @os_ClrLCDFull")
        .replace("void @os_ClrLCD", "cc102 void @os_ClrLCD")
        .replace("void @os_ClrTxtShd", "cc102 void @os_ClrTxtShd")
        // ti/ui.h
        .replace("void @os_RunIndicOn", "cc102 void @os_RunIndicOn")
        .replace("void @os_RunIndicOff", "cc102 void @os_RunIndicOff")
        .replace("void @os_DrawStatusBar", "cc102 void @os_DrawStatusBar")
        // ti/vars.h
        .replace("void @os_ArcChk", "cc102 void @os_ArcChk")
        .replace("void @os_DelRes", "cc102 void @os_DelRes");
    std::fs::write(file, patched_content).expect("Erreur lors de l'écriture du fichier patché");
}

fn should_skip_next(arg: &str) -> bool {
    matches!(arg, "-flavor" | "-L" | "-Bstatic" | "-Bdynamic" | "-z")
}

fn ignore(arg: &str) -> bool {
    if arg.starts_with("-plugin-opt") {
        return true;
    }
    matches!(arg, "--gc-sections" | "--as-needed" | "--eh-frame-hdr")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut iter = args.iter();
    let mut files = Vec::new();
    let mut output = String::new();
    iter.next(); // Skip program name
    while let Some(arg) = iter.next() {
        if should_skip_next(arg) {
            iter.next();
            continue;
        }
        if ignore(arg) {
            continue;
        }
        if arg == "-o" {
            if let Some(out) = iter.next() {
                output = out.clone();
            }
            continue;
        } else {
            files.push(arg.clone());
        }
    }
    println!("Fichiers: {:?}, Sortie: {}", files, output);
    create_dirs();
    // convert to llvm-ir files
    for file in &files {
        let out_file_name = if file.ends_with(".o") {
            Path::new(file).file_stem().unwrap().to_string_lossy().to_string()
        } else {
            println!("❌ Format de fichier non supporté: {}", file);
            std::process::exit(1);
        };
        if out_file_name == "symbols" {
            continue; // skip symbols.o
        }
        let out_file = format!("incremental/{}.ll", out_file_name);
        // copy file
        std::fs::copy(file, &out_file).expect("Erreur lors de la copie du fichier");
        println!("Conversion de {} en {}", file, out_file);
        let cmd = std::process::Command::new("llvm-dis")
            .arg(file)
            .arg("-o")
            .arg(&out_file)
            .output();
        let output = cmd.expect("Erreur lors de l'exécution de llvm-dis");
        if !output.status.success() {
            println!("❌ Erreur lors de la conversion de {}: {}", file, String::from_utf8_lossy(&output.stderr));
            std::process::exit(1);
        }
        patch_file(&out_file);
    }
    
    std::process::exit(1);
}

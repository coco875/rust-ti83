use std::path::Path;

fn create_dirs() {
    let _ = std::fs::create_dir_all("incremental");
}

fn patch_asm(file: &str) {
    let content = std::fs::read_to_string(file).expect("Error reading file");
    let patched_content = content
        .replace(".unlikely.", "");

    std::fs::write(file, patched_content).expect("Error writing patched file");
}

fn should_skip_next(arg: &str) -> bool {
    matches!(arg, "-flavor" | "-L" | "-z")
}

fn ignore(arg: &str) -> bool {
    if arg.starts_with("-plugin-opt") {
        return true;
    }
    matches!(arg, "--gc-sections" | "--as-needed" | "--eh-frame-hdr" | "--strip-debug" | "-Bstatic" | "-Bdynamic")
}

fn run_command(mut cmd: std::process::Command) -> bool {
    match cmd.status() {
        Ok(status) if status.success() => true,
        Ok(status) => {
            eprintln!("Command failed with code: {}", status.code().unwrap_or(-1));
            false
        }
        Err(e) => {
            eprintln!("Error executing command: {}", e);
            false
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut iter = args.iter();
    let mut files = Vec::new();
    let mut input_c_files = Vec::new();
    let mut input_bc_files = Vec::new();
    let mut output = String::new();
    let elf_name = "output";
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
    println!("Files: {:?}, Output: {}", files, output);
    create_dirs();

    let cedev = std::path::absolute("CEdev").unwrap().to_string_lossy().to_string();

    // convert to llvm-ir files
    for file in &files {
        if file.ends_with(".o") {
            let out_file_name = Path::new(file).file_stem().unwrap().to_string_lossy().to_string();
            if out_file_name == "symbols" {
                continue; // skip symbols.o
            }
            let out_file = format!("incremental/{}.cbe.c", out_file_name);
            // copy file
            std::fs::copy(file, &out_file).expect("Erreur lors de la copie du fichier");
            println!("convert from {} into {}", file, out_file);
            let mut cmd = std::process::Command::new("./llvm-cbe");
            cmd.arg(file);
            cmd.arg("-O3");
            cmd.arg("-o");
            cmd.arg(&out_file);
            if !run_command(cmd) {
                std::process::exit(1);
            }
            input_c_files.push(out_file);
        } else if file.ends_with(".c") {
            let out_file = file.to_string();
            input_c_files.push(out_file);
        } else {
            if file.contains("libcompiler_builtins") {
                println!("skip libcompiler_builtins");
                continue;
            }
            println!("unknown file type: {}", file);
            std::process::exit(1);
        };
        
    }

    println!("Compile to bc");
    for name in &input_c_files {
        let mut cmd = std::process::Command::new(&format!("{}/bin/ez80-clang", cedev));
        let mut args: Vec<String> = vec![
            "-c".to_string(),
            "-emit-llvm".to_string(),
            "-Wall".to_string(),
            "-Wextra".to_string(),
            "-Wno-incompatible-library-redeclaration".to_string(),
            "-Wno-unused-parameter".to_string(),
            "-Wno-unused-variable".to_string(),
            "-Wno-unused-function".to_string(),
            "-Wno-parentheses-equality".to_string(),
            "-Wno-unused-but-set-variable".to_string(),
            "-Oz".to_string(),
            "-nostdinc".to_string(),
            "-isystem".to_string(),
            format!("{}/include", cedev),
            "-D__TICE__=1".to_string(),
            "-fno-threadsafe-statics".to_string(),
            "-Xclang".to_string(),
            "-fforce-mangle-main-argc-argv".to_string(),
            "-mllvm".to_string(),
            "-profile-guided-section-prefix=false".to_string(),
            name.clone(),
            "-o".to_string(),
        ];
        let output_path = format!("./incremental/{}.bc", Path::new(name).file_stem().unwrap().to_string_lossy());
        args.push(output_path.clone());
        cmd.args(&args);
        if !run_command(cmd) {
            std::process::exit(1);
        }
        input_bc_files.push(output_path);
    }

    let mut cmd = std::process::Command::new(&format!("{}/bin/ez80-link", cedev));
    let mut args: Vec<String> = Vec::new();
    args.push("--only-needed".to_string());
    args.push("--internalize".to_string());
    for name in input_bc_files {
        args.push(name);
    }
    args.push("-o".to_string());
    let output_path = format!("./incremental/{}.bc", elf_name);
    args.push(output_path.clone());
    cmd.args(&args);
    if !run_command(cmd) {
        std::process::exit(1);
    }
    
    println!("Compile to asm");
    let mut cmd = std::process::Command::new(&format!("{}/bin/ez80-clang", cedev));
    let mut args = vec![
        "-S",
        "-Oz",
    ];
    args.push(&output_path);
    args.push("-o");
    let output_path = format!("./incremental/{}.s", elf_name);
    args.push(&output_path);
    cmd.args(&args);
    if !run_command(cmd) {
        println!("Fail to create asm");
        std::process::exit(1);
    }
    patch_asm(&output_path);
    let mut cmd = std::process::Command::new(&format!("{}/bin/fasmg", cedev));
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
        "-i", &format!("include '{}/meta/linker_script'", cedev),
        "-i", "range .bss $D052C6 : $D13FD8",
        "-i", "provide __stack = $D1A87E",
        "-i", "locate .header at $D1A87F",
        "-i", "map",
        "-i", &format!("source \"{}/lib/crt/crt0.src\", \"./incremental/{}.s\"", cedev, elf_name),
        "-i", &format!("library \"{}/lib/libload/fatdrvce.lib\", \"{}/lib/libload/fileioc.lib\", \"{}/lib/libload/fontlibc.lib\", \"{}/lib/libload/graphx.lib\", \"{}/lib/libload/keypadc.lib\", \"{}/lib/libload/msddrvce.lib\", \"{}/lib/libload/srldrvce.lib\", \"{}/lib/libload/usbdrvce.lib\"", cedev, cedev, cedev, cedev, cedev, cedev, cedev, cedev),
        &format!("incremental/{}.bin", elf_name)
    ]);
    if !run_command(cmd) {
        println!("Fail to create bin");
        std::process::exit(1);
    }

    let mut cmd = std::process::Command::new(&format!("{}/bin/convbin", cedev));
    cmd.args(&[
        "--oformat", "8xp",
        "--uppercase",
        "--name", elf_name,
        "--input", &format!("incremental/{}.bin", elf_name),
        "--output", &output
    ]);
    if !run_command(cmd) {
        println!("Fail to create 8xp");
        std::process::exit(1);
    }
}

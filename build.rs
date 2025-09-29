use std::path::Path;
use std::process::Command;

fn download_file(url: &str, output: &str) -> bool {
    let status = if cfg!(target_os = "windows") || cfg!(target_os = "macos") {
        Command::new("curl")
            .args(&["-L", "-o", output, url])
            .status()
    } else {
        Command::new("wget")
            .args(&["-q", "-O", output, url])
            .status()
    };

    match status {
        Ok(status) if status.success() => true,
        Ok(status) => {
            eprintln!("Download failed with code: {}", status.code().unwrap_or(-1));
            false
        }
        Err(e) => {
            eprintln!("Error executing download command: {}", e);
            false
        }
    }
}

fn download_cedev() {
    let cedev = "./CEdev";
    if !Path::new(cedev).exists() {
        // should be improve to support windows
        if std::env::consts::OS == "linux" {
            if !download_file("https://github.com/CE-Programming/toolchain/releases/download/v13.0/CEdev-Linux.tar.gz", "CEdev-Linux.tar.gz") {
                eprintln!("Failed to download CEdev");
                std::process::exit(1);
            }
            let status = std::process::Command::new("tar")
                .args(&["-xzf", "CEdev-Linux.tar.gz"])
                .status()
                .expect("Failed to execute tar");
            if !status.success() {
                eprintln!("Failed to extract CEdev");
                std::process::exit(1);
            }
            let _ = std::fs::remove_file("CEdev-Linux.tar.gz");
        } else if std::env::consts::OS == "macos" {
            if std::env::consts::ARCH == "aarch64" {
                if !download_file("https://github.com/CE-Programming/toolchain/releases/download/v13.0/CEdev-macOS-arm.dmg", "CEdev-macOS.dmg") {
                    eprintln!("Failed to download CEdev");
                    std::process::exit(1);
                }
            } else {
                if !download_file("https://github.com/CE-Programming/toolchain/releases/download/v13.0/CEdev-macOS-intel.dmg", "CEdev-macOS.dmg") {
                    eprintln!("Failed to download CEdev");
                    std::process::exit(1);
                }
            }
            let status = std::process::Command::new("7z")
                .args(&["x", "CEdev-macOS.dmg", "CE Programming Toolchain/CEdev/"])
                .status()
                .expect("Failed to execute 7z");
            if !status.success() {
                eprintln!("Failed to extract CEdev");
                std::process::exit(1);
            }
            let status = std::process::Command::new("mv")
                .args(&["CE Programming Toolchain/CEdev", "."])
                .status()
                .expect("Failed to execute mv");
            if !status.success() {
                eprintln!("Failed to move CEdev");
                std::process::exit(1);
            }
            let status = std::process::Command::new("chmod")
                .args(&["-R", "+x", "CEdev/bin"])
                .status()
                .expect("Failed to execute chmod");
            if !status.success() {
                eprintln!("Failed to chmod CEdev");
                std::process::exit(1);
            }

            let _ = std::fs::remove_file("CEdev-macOS.dmg");
            let _ = std::fs::remove_dir_all("CE Programming Toolchain");
        } else if std::env::consts::OS == "windows" {
            if !download_file("https://github.com/CE-Programming/toolchain/releases/download/v13.0/CEdev-Windows.zip", "CEdev-Windows.zip") {
                eprintln!("Failed to download CEdev");
                std::process::exit(1);
            }
            let status = std::process::Command::new("tar")
                .args(&["-xf", "CEdev-Windows.zip"])
                .status()
                .expect("Failed to execute tar");
            if !status.success() {
                eprintln!("Failed to extract CEdev");
                std::process::exit(1);
            }
        } else {
            eprintln!("CEdev not found and automatic download is only supported on Linux, Macos and Windows.");
            std::process::exit(1);
        }
    }
}

fn download_llvm_cbe() {
    let llvm_cbe = "./llvm-cbe";
    if !Path::new(llvm_cbe).exists() {
        // should be improve to support windows
        if std::env::consts::OS == "linux" {
            if !download_file("https://github.com/coco875/llvm-cbe/releases/download/0.0.1/llvm-cbe-linux-x64.zip", "llvm-cbe-ubuntu-latest-build.zip") {
                eprintln!("Failed to download llvm-cbe");
                std::process::exit(1);
            }
            let status = std::process::Command::new("unzip")
                .args(&["-q", "llvm-cbe-ubuntu-latest-build.zip"])
                .status()
                .expect("Failed to execute unzip");
            if !status.success() {
                eprintln!("Failed to extract llvm-cbe");
                std::process::exit(1);
            }
            let _ = std::fs::remove_file("llvm-cbe-ubuntu-latest-build.zip");
        } else if std::env::consts::OS == "macos" {
            if std::env::consts::ARCH == "aarch64" {
                if !download_file("https://github.com/coco875/llvm-cbe/releases/download/0.0.1/llvm-cbe-macos-arm.zip", "llvm-cbe-macos-build.zip") {
                    eprintln!("Failed to download llvm-cbe");
                    std::process::exit(1);
                }
            } else {
                if !download_file("https://github.com/coco875/llvm-cbe/releases/download/0.0.1/llvm-cbe-macos-intel.zip", "llvm-cbe-macos-build.zip") {
                    eprintln!("Failed to download llvm-cbe");
                    std::process::exit(1);
                }
            }

            let status = std::process::Command::new("unzip")
                .args(&["-q", "llvm-cbe-macos-build.zip"])
                .status()
                .expect("Failed to execute unzip");
            if !status.success() {
                eprintln!("Failed to extract llvm-cbe");
                std::process::exit(1);
            }
            let _ = std::fs::remove_file("llvm-cbe-macos-build.zip");
        } else if std::env::consts::OS == "windows" {
            if !download_file("https://github.com/coco875/llvm-cbe/releases/download/0.0.1/llvm-cbe-windows-x64.zip", "llvm-cbe-windows-build.zip") {
                eprintln!("Failed to download llvm-cbe");
                std::process::exit(1);
            }
            let status = std::process::Command::new("tar")
                .args(&["-xf", "llvm-cbe-windows-build.zip"])
                .status()
                .expect("Failed to execute tar");
            if !status.success() {
                eprintln!("Failed to extract llvm-cbe");
                std::process::exit(1);
            }
            let _ = std::fs::remove_file("llvm-cbe-windows-build.zip");
        } else {
            eprintln!("llvm-cbe not found and automatic download is only supported on Linux and macOS ARM.");
            std::process::exit(1);
        }
    }
}

fn main() {
    // check if we compile for tice
    let target = std::env::var("TARGET").unwrap();
    if target.contains("tice") {
        println!("cargo:warning=we are building for tice");
        download_cedev();
        download_llvm_cbe();
        let status = Command::new("rustc")
            .args(&["ti83-fake-linker.rs", "-o", "ti83-fake-linker"])
            .status()
            .expect("Failed to execute rustc");
        if !status.success() {
            panic!("Failed to build ti83-fake-linker");
        }
        println!("cargo:rerun-if-changed=ti83-fake-linker.rs");
        println!("cargo:rustc-link-arg-bins=src/tice/wrapper.c");
    }
}

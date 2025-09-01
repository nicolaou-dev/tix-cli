use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../tix/tix.h");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    // Determine the library file name and download URL
    let (lib_name, download_name) = match (target_os.as_str(), target_arch.as_str()) {
        ("macos", "aarch64") => ("libtix.a", "libtix-macos-aarch64.tar.gz"),
        ("macos", "x86_64") => ("libtix.a", "libtix-macos-x86_64.tar.gz"),
        ("linux", "x86_64") => ("libtix.a", "libtix-linux-x86_64.tar.gz"),
        ("windows", "x86_64") => ("tix.lib", "libtix-windows-x86_64.zip"),
        _ => panic!("Unsupported platform: {target_os}-{target_arch}"),
    };

    let version = "v0.0.9";
    let download_url =
        format!("https://github.com/nicolaou-dev/tix/releases/download/{version}/{download_name}");

    // Download the library if it doesn't exist
    let lib_path = out_dir.join(lib_name);
    if !lib_path.exists() {
        println!("cargo:warning=Downloading tix library from {download_url}");

        // Download the archive
        let archive_path = out_dir.join(download_name);
        download_file(&download_url, &archive_path);

        // Extract the library
        extract_library(&archive_path, &out_dir, lib_name);

        // Clean up archive
        fs::remove_file(archive_path).ok();
    }

    // Tell cargo where to find the library
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=tix");

    // Generate bindings from tix.h using bindgen
    let header_path = out_dir.join("tix.h");
    if header_path.exists() {
        let bindings = bindgen::Builder::default()
            .header(header_path.to_str().unwrap())
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .generate_comments(true) // Preserve documentation comments
            .generate()
            .expect("Unable to generate bindings");

        bindings
            .write_to_file(out_dir.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    } else {
        panic!("tix.h not found in downloaded archive!");
    }
}

fn download_file(url: &str, path: &PathBuf) {
    let mut response = ureq::get(url)
        .call()
        .unwrap_or_else(|_| panic!("Failed to download from {url}"));

    let mut file = fs::File::create(path).expect("Failed to create file");

    let mut reader = response.body_mut().as_reader();
    std::io::copy(&mut reader, &mut file).expect("Failed to write file");
}

fn extract_library(archive_path: &PathBuf, out_dir: &Path, lib_name: &str) {
    let archive_str = archive_path.to_str().unwrap();

    if archive_str.ends_with(".tar.gz") {
        // Extract tar.gz
        let tar_gz = fs::File::open(archive_path).unwrap();
        let tar = flate2::read::GzDecoder::new(tar_gz);
        let mut archive = tar::Archive::new(tar);

        for entry in archive.entries().unwrap() {
            let mut entry = entry.unwrap();
            let path = entry.path().unwrap();
            let filename = path.file_name().unwrap();

            // Extract both library and header
            if filename == lib_name || filename == "tix.h" {
                let dest_path = out_dir.join(filename);
                entry.unpack(&dest_path).unwrap();
            }
        }
    } else if archive_str.ends_with(".zip") {
        // Extract zip (for Windows)
        let file = fs::File::open(archive_path).unwrap();
        let mut archive = zip::ZipArchive::new(file).unwrap();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let name = file.name();

            // Extract both library and header (and .lib for Windows)
            if name == lib_name || name == "tix.h" || name == "tix.lib" {
                let dest_path = out_dir.join(name);
                let mut dest_file = fs::File::create(&dest_path).unwrap();
                std::io::copy(&mut file, &mut dest_file).unwrap();
            }
        }
    }
}

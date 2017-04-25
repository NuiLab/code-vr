extern crate curl;

use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

use curl::easy::Easy;

/// Fetch the platform specific OpenVR 1.0.1 DLL and include it with the build
fn fetch_openvr_sdk() {

    // Get the platform specific version of the openVR DLL
    let commit101 = String::from("e1507a27547d22a680153862865d40b90fad8c75");
    let mut os = String::from("win");
    let mut arch = String::from("64");
    let mut file_type = String::from("dll");
    let mut prefix = String::from("");

    if cfg!(target_arch = "x86") {
        arch = String::from("32");
    }

    if cfg!(target_os = "linux") {
        os = String::from("linux");
        prefix = String::from("lib");
        file_type = String::from("so");
    }

    let mut file = String::new();
    file.push_str(prefix.as_str());
    file.push_str("openvr_api.");
    file.push_str(file_type.as_str());

    // if this file doesn't already exist in build directory

    let mut out_dir = String::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    out_dir.push_str("/target/");
    out_dir.push_str(env::var("PROFILE").unwrap().as_str());

    let mut file_path_str = String::new();
    file_path_str.push_str(out_dir.as_str());
    file_path_str.push_str("/");
    file_path_str.push_str(file.as_str());

    let file_path = Path::new(file_path_str.as_str());

    if !file_path.exists() {

        let mut download = String::from("https://raw.githubusercontent.com/ValveSoftware/openvr/");
        download.push_str(commit101.as_str());
        download.push_str("/bin/");
        download.push_str(os.as_str());
        download.push_str(arch.as_str());
        download.push_str("/");
        download.push_str(file.as_str());

        // Fetch the file from GitHub
        let mut open = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path_str.as_str()).unwrap();

        let mut handle = Easy::new();

        handle.url(download.as_str()).unwrap();
        handle
            .write_function(move |data| {
                Ok(open.write(data).unwrap())})
            .unwrap();
        handle.perform().unwrap();
    }
}

fn main() {
    fetch_openvr_sdk();
}


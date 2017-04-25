use std::env;
use std::fs;

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
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut download = String::from("https://github.com/ValveSoftware/openvr/raw/");
    download.push_str(commit101.as_str());
    download.push_str("/bin/");
    download.push_str(os.as_str());
    download.push_str(arch.as_str());
    download.push_str("/");
    download.push_str(file.as_str());

    println!("{}", out_dir);

    // Fetch the file from GitHub

}

fn main() {
    fetch_openvr_sdk();
}

extern crate serde_json;

mod window;
mod graphics;
mod sound;

use std::fs::OpenOptions;
use std::io::{Read, Write};
use serde::Serialize;

use self::window::WindowConfig;
use self::graphics::GraphicsConfig;
use self::sound::SoundConfig;
use std::fs;

/// Configuration object passed to the renderer.
#[derive(Serialize, Deserialize)]
struct Config {
  window: WindowConfig,
  graphics: GraphicsConfig,
  sound: SoundConfig
}

/// Tries to read Config JSON data from the working directory, returns either defaults or what was in file.
pub fn read() -> Config {

    let mut default_config: Config = serde_json::from_str("").unwrap();

    // Create codevr/ folder in WORKING_DIRECTORY
    fs::create_dir("%APPDATA%/codevr").unwrap_or(());

    let open = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("%APPDATA%/codevr/config.json");

    let mut contents = String::new();

    let mut file = match open {
        Err(_) => return default_config,
        Ok(file) => file,
    };

    match file.read_to_string(&mut contents) {
        Err(_) => return default_config,
        Ok(c) => (),
    }

    if contents.is_empty() {

        contents.insert_str(0, serde_json::to_string_pretty(&default_config).unwrap().as_str());

        match file.write_all(contents.as_bytes()) {

            Err(_) => return default_config,
            Ok(_) => (),
        }
    }

    serde_json::from_str(contents.as_str()).unwrap()
}
use crate::data_types::Settings;
use std::{fs::File, io::{Read, Write}};

fn create_default_app_config() {
    save(&Settings::default());
}

pub fn save(settings: &Settings) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("settings.json").expect("Failed to get file handle!");
    serde_json::to_writer(file, &settings).expect("Failed to crate file!");

    Ok(())
}

pub fn load() -> Result<Settings, Box<dyn std::error::Error>> {
    let mut settings = Settings::default();
    match File::open("settings.json") {
        Ok(mut file) => {
            let mut temp = String::new();
            file.read_to_string(&mut temp)?;
            settings = serde_json::from_str(&temp).unwrap();
        }
        Err(error) => {
            if let std::io::ErrorKind::NotFound = error.kind() {
                dbg!("[INFO] No settings, initalizing default config.");
                create_default_app_config();
            }
        }
    }

    Ok(settings)
}

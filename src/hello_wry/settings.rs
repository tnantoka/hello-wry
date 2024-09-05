use serde::{Deserialize, Serialize};
use dirs::data_local_dir;
use std::fs::{File, create_dir_all};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub color: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            color: "ffffff".to_string(),
        }
    }
}

impl Settings {
    pub fn init() -> Self {
        match Settings::restore() {
            Ok(settings) => settings,
            _ => Settings::default(),
        }
    }

    fn restore() -> Result<Self, std::io::Error> {
        let file = File::open(Settings::json_path())?;
        let reader = BufReader::new(file);
        let settings = serde_json::from_reader(reader)?;
        Ok(settings)
    }

    fn json_path() -> PathBuf {
        Settings::dir_path().join("settings.json")
    }

    fn dir_path() -> PathBuf {
        data_local_dir().unwrap().join("hello-wry")
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        create_dir_all(Settings::dir_path())?;
        let file = File::create(Settings::json_path())?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, self)?;
        Ok(())
    }
}

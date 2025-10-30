use std::{error::Error, fs, path::PathBuf};

use serde::Deserialize;

use crate::catrina::Pins;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub arduino_serial_port: String,
    pub open_see_face_address: String,
    pub audio_folder: Option<PathBuf>,
    pub pins: Pins,
}

pub fn read_config_from_file(path: &str) -> Result<Config, Box<dyn Error>> {
    let json_text = fs::read_to_string(path)?;
    let config = serde_json::from_str(&json_text)?;
    Ok(config)
}

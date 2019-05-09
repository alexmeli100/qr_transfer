use serde::{Deserialize, Serialize};
use dirs;
use std::path::PathBuf;
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub iface: String,
    pub port: usize
}

pub fn config_file() -> PathBuf {
    let home = dirs::home_dir().unwrap();

    home.join("qr-filetransfer.json")
}

pub fn new() -> Config {
    let p = config_file();
    let cfg = fs::read_to_string(p).expect("Error reading config file");

    serde_json::from_str(&cfg).unwrap()

}
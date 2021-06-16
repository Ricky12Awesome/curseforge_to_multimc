use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Settings {
  mmc_directory: Option<String>,
  cf_directory: Option<String>
}

impl Settings {
  pub fn load() -> Self {
    let path = "settings.json";

    match File::open(path) {
      Ok(file) => serde_json::from_reader(&file).unwrap_or_default(),
      Err(_) => Self::default()
    }
  }
}
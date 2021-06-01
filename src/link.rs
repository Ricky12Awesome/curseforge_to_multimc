use std::fmt::Formatter;

use serde::{Deserialize, Serialize};

use crate::modpack::CFModPack;
use crate::util::{CurseForgeDirectory, MultiMCDirectory};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
struct CFModPackManifest {
  minecraft: CFMinecraftJson,
  name: String,
  version: String,
  author: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CFMinecraftJson {
  version: String,
  #[serde(alias = "modLoaders")] mod_loaders: Vec<CFModLoadersJson>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CFModLoadersJson {
  id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MMCModPackManifest {}

#[derive(Debug)]
pub struct LinkError {
  msg: &'static str,
  mmc: MultiMCDirectory,
  cf:  CurseForgeDirectory,
  selected: CFModPack,
}

impl std::error::Error for LinkError {

}

fn err<'a>(
  msg: &'static str,
  mmc: MultiMCDirectory,
  cf: CurseForgeDirectory,
  selected: CFModPack,
) -> Result {
  Err(Box::new(LinkError { msg, mmc, cf, selected }))
}

impl std::fmt::Display for LinkError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.msg)
  }
}

pub type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub fn link(
  mmc: MultiMCDirectory,
  cf: CurseForgeDirectory,
  selected: CFModPack,
) -> Result {
  let file = selected.path().join("manifest.json");
  let file = File::open(file)?;
  let bytes = file.bytes();
  let bytes = bytes.map(|it| it.unwrap_or_default()).collect::<Vec<_>>();
  let json = serde_json::from_slice::<CFModPackManifest>(&bytes)?;

  dbg!(&json);

  err("Failed.", mmc, cf, selected)
}
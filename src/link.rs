use std::fmt::Formatter;
use std::fs::{create_dir, File, remove_dir_all};
use std::io::{Read, Write};

use serde::{Deserialize, Serialize};

use crate::modpack::CFModPack;
use crate::directories::{CurseForgeDirectory, Directory, MultiMCDirectory};

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

enum CFMinecraftLoaderVersion {
  Forge(String),
  Fabric(String),
  Unknown,
}

impl CFMinecraftJson {
  fn version(&self) -> CFMinecraftLoaderVersion {
    match self.mod_loaders[0].id.split_once("-") {
      Some(("forge", version)) => CFMinecraftLoaderVersion::Forge(version.to_string()),
      Some(("fabric", version)) => CFMinecraftLoaderVersion::Fabric(version.to_string()),
      _ => CFMinecraftLoaderVersion::Unknown
    }
  }
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
  cf: CurseForgeDirectory,
  selected: CFModPack,
}

impl std::error::Error for LinkError {}

fn err<'a>(
  msg: &'static str,
  mmc: MultiMCDirectory,
  cf: CurseForgeDirectory,
  selected: CFModPack,
) -> Result<()> {
  Err(Box::new(LinkError { msg, mmc, cf, selected }))
}

impl std::fmt::Display for LinkError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.msg)
  }
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn create_mmc_instance_cfg(manifest: &CFModPackManifest) -> String {
  let mut str = String::new();

  str.push_str("InstanceType=OneSix\n");
  str.push_str("JoinServerOnLaunch=false\n");
  str.push_str("OverrideCommands=false\n");
  str.push_str("OverrideConsole=false\n");
  str.push_str("OverrideGameTime=false\n");
  str.push_str("OverrideJavaArgs=false\n");
  str.push_str("OverrideJavaLocation=false\n");
  str.push_str("OverrideMemory=false\n");
  str.push_str("OverrideNativeWorkarounds=false\n");
  str.push_str("OverrideWindow=false\n");
  str.push_str("iconKey=default\n");
  str.push_str(format!("name={}\n", manifest.name).as_str());
  str.push_str("notes=\n");

  str
}

fn create_mmc_pack_json(manifest: &CFModPackManifest) -> serde_json::Value {
  let minecraft_component = |manifest: &CFModPackManifest| {
    serde_json::json!({
      "cachedName": "Minecraft",
      "cachedRequires": [],
      "cachedVersion": manifest.minecraft.version,
      "important": true,
      "uid": "net.minecraft",
      "version": manifest.minecraft.version
    })
  };

  let version_component = |manifest: &CFModPackManifest| {
    match manifest.minecraft.version() {
      CFMinecraftLoaderVersion::Fabric(version) => serde_json::json!({
        "cachedName": "Fabric Loader",
        "uid": "net.fabricmc.fabric-loader",
        "version": version
      }),
      CFMinecraftLoaderVersion::Forge(version) => serde_json::json!({
        "cachedName": "Forge",
        "uid": "net.minecraftforge",
        "version": version
      }),
      _ => serde_json::json!({})
    }
  };

  serde_json::json!(
    {
      "components": [
        minecraft_component(manifest),
        version_component(manifest)
      ],
      "formatVersion": 1
    }
  )
}

fn manifest(selected: &CFModPack) -> Result<CFModPackManifest> {
  let path = selected.path().join("manifest.json");
  let file = File::open(path)?;
  let bytes = file
    .bytes()
    .map(|it| it.unwrap_or_default())
    .collect::<Vec<_>>();

  Ok(serde_json::from_slice::<CFModPackManifest>(&bytes)?)
}

pub fn unlink(
  mmc: MultiMCDirectory,
  selected: CFModPack,
) -> Result<()> {
  let manifest = manifest(&selected)?;
  let mmc_path = mmc.path.join(&manifest.name);

  remove_dir_all(mmc_path)?;

  Ok(())
}

pub fn link(
  mmc: MultiMCDirectory,
  cf: CurseForgeDirectory,
  selected: CFModPack,
) -> Result<()> {
  let manifest = manifest(&selected)?;
  let mmc_pack = serde_json::to_string_pretty(&create_mmc_pack_json(&manifest))?;
  let mmc_cfg = create_mmc_instance_cfg(&manifest);
  let mmc_path = mmc.path().join(&manifest.name);

  if mmc_path.exists() {
    return err("A folder with that name already exists", mmc, cf, selected);
  }

  create_dir(&mmc_path)?;

  let mut mmc_cfg_file = File::create(mmc_path.join("instance.cfg"))?;
  let mut mmc_pack_file = File::create(mmc_path.join("mmc-pack.json"))?;

  mmc_cfg_file.write(mmc_cfg.as_bytes())?;
  mmc_pack_file.write(mmc_pack.as_bytes())?;

  match symlink::symlink_dir(selected.path(), mmc_path.join("minecraft")) {
    Ok(_) => Ok(()),
    Err(_) => {
      remove_dir_all(&mmc_path)?;
      err("No permission to create symlink (Needs admin perms)", mmc, cf, selected)
    }
  }
}
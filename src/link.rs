use std::fmt::Formatter;
use std::fs::{create_dir, File, remove_dir_all};

use serde::{Deserialize, Serialize};

use crate::directories::{CurseForgeDirectory, Directory, MultiMCDirectory};
use crate::modpack::CFModPack;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
struct CFMinecraftInstance {
  name: String,
  #[serde(alias = "baseModLoader")] loader: CFBaseModLoader,
}

#[derive(Debug, Serialize, Deserialize)]
struct CFBaseModLoader {
  name: String,
  #[serde(alias = "forgeVersion")] version: String,
  #[serde(alias = "minecraftVersion")] mc_version: String,
}

enum CFMinecraftLoaderVersion {
  Forge(String),
  Fabric(String),
  Unknown,
}

impl CFBaseModLoader {
  fn version(&self) -> CFMinecraftLoaderVersion {
    match self.name.split_once("-") {
      Some(("forge", _)) => CFMinecraftLoaderVersion::Forge(self.version.clone()),
      Some(("fabric", _)) => CFMinecraftLoaderVersion::Fabric(self.version.clone()),
      _ => CFMinecraftLoaderVersion::Unknown
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
struct CFModLoadersJson {
  id: String,
}

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

fn create_mmc_instance_cfg(instance: &CFMinecraftInstance) -> String {
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
  str.push_str(format!("name={}\n", instance.name).as_str());
  str.push_str("notes=\n");

  str
}

fn create_mmc_pack_json(instance: &CFMinecraftInstance) -> serde_json::Value {
  let minecraft_component = |instance: &CFMinecraftInstance| {
    serde_json::json!({
      "cachedName": "Minecraft",
      "cachedRequires": [],
      "cachedVersion": instance.loader.mc_version,
      "important": true,
      "uid": "net.minecraft",
      "version": instance.loader.mc_version
    })
  };

  let version_component = |instance: &CFMinecraftInstance| {
    match instance.loader.version() {
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
        minecraft_component(instance),
        version_component(instance)
      ],
      "formatVersion": 1
    }
  )
}

fn get_cf_instance(selected: &CFModPack) -> Result<CFMinecraftInstance> {
  let path = selected.path().join("minecraftinstance.json");

  Ok(serde_json::from_reader(&File::open(&path)?)?)
}

pub fn unlink(
  mmc: MultiMCDirectory,
  selected: CFModPack,
) -> Result<()> {
  let instance = get_cf_instance(&selected)?;
  let mmc_path = mmc.path.join(&instance.name);

  remove_dir_all(mmc_path)?;

  Ok(())
}

pub fn link(
  mmc: MultiMCDirectory,
  cf: CurseForgeDirectory,
  selected: CFModPack,
) -> Result<()> {
  let instance = get_cf_instance(&selected)?;
  let mmc_pack = serde_json::to_string_pretty(&create_mmc_pack_json(&instance))?;
  let mmc_cfg = create_mmc_instance_cfg(&instance);
  let mmc_path = mmc.path().join(&instance.name);

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
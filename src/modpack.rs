use std::fmt::Formatter;
use std::path::PathBuf;

use crate::directories::{CurseForgeDirectory, Directory, MultiMCDirectory};
use crate::link::get_cf_instance;

#[derive(Default, Debug, Clone, PartialOrd, PartialEq)]
pub struct ModPack {
  pub cf_dir: Option<PathBuf>,
}

impl std::fmt::Display for ModPack {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(
      self.cf_dir
        .clone().unwrap_or_default()
        .file_name().unwrap_or_default()
        .to_str().unwrap_or_default()
    )
  }
}

impl Eq for ModPack {}

impl ModPack {
  pub fn cf_path(&self) -> PathBuf {
    self.cf_dir.clone().unwrap_or_default()
  }

  pub fn mmc_path(&self, mmc: &MultiMCDirectory) -> Option<PathBuf> {
    let instance = get_cf_instance(self).ok()?;
    let path = mmc.path.join(&instance.name);

    if path.exists() {
      Some(path)
    } else {
      None
    }
  }

  pub fn is_linked(&self, mmc: &MultiMCDirectory) -> bool {
    self.mmc_path(mmc).is_some()
  }

  pub fn list(cf: CurseForgeDirectory, selected: &mut Option<ModPack>) -> Vec<ModPack> {
    let path = cf.path();

    path
      .read_dir()
      .map(|it| it
        .map(|it| ModPack {
          cf_dir: it.map(|it| it.path()).ok()
        })
        .collect::<Vec<_>>()
      )
      .map_err(|_| *selected = None)
      .unwrap_or_default()
  }
}
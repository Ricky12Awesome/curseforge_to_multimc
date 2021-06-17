use std::fmt::Formatter;
use std::path::PathBuf;

use crate::directories::{CurseForgeDirectory, Directory};

#[derive(Default, Debug, Clone, PartialOrd, PartialEq)]
pub struct CFModPack {
  pub dir: Option<PathBuf>,
}

impl std::fmt::Display for CFModPack {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(
      self.dir
        .clone().unwrap_or_default()
        .file_name().unwrap_or_default()
        .to_str().unwrap_or_default()
    )
  }
}

impl Eq for CFModPack {}

impl CFModPack {
  pub fn path(&self) -> PathBuf {
    self.dir.clone().unwrap_or_default()
  }

  pub fn list(dir: CurseForgeDirectory, selected: &mut Option<CFModPack>) -> Vec<CFModPack> {
    let path = dir.path();

    path
      .read_dir()
      .map(|it| it
        .map(|it| CFModPack {
          dir: it.map(|it| it.path()).ok()
        })
        .collect::<Vec<_>>()
      )
      .map_err(|_| *selected = None)
      .unwrap_or_default()
  }
}
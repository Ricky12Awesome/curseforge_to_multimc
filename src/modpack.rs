use std::path::{Path, PathBuf};
use std::fmt::Formatter;
use crate::CurseForgeToMultiMC;
use crate::make_multi_mut_ref;
use crate::util::Directory;

#[derive(Default, Debug, Clone)]
pub struct CFModPackCache {
  path: Option<Box<Path>>,
  storage: Vec<CFModPack>,
}

#[derive(Default, Debug, Clone, PartialOrd, PartialEq)]
pub struct CFModPack {
  pub dir: Option<PathBuf>,
}

impl std::fmt::Display for CFModPack {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.dir.clone().unwrap_or_default().file_name().unwrap_or_default().to_str().unwrap_or_default())
  }
}

impl Eq for CFModPack {}

impl CFModPack {
  pub fn path(&self) -> PathBuf {
    self.dir.clone().unwrap_or_default()
  }

  pub fn list<'a>(app: &'a mut CurseForgeToMultiMC<'a>) -> &'a [CFModPack] {
    let app_ref = make_multi_mut_ref!(app, CurseForgeToMultiMC);
    let dir = &app.cf_d;
    let cache = &mut app.pick_cf_mp_cache;
    let path = dir.path();

    if let Some(cache_path) = &cache.path {
      if cache_path.as_ref() == path {
        return &cache.storage;
      }
    }

    cache.storage = path
      .read_dir()
      .map(|it| it
        .map(|it| CFModPack {
          dir: it.map(|it| it.path()).ok()
        })
        .collect::<Vec<_>>()
      )
      .map_err(|_| app_ref.selected_cf_mp = None)
      .unwrap_or_default();

    &cache.storage
  }
}
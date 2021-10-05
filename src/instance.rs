use std::ops::Deref;
use std::path::{Path, PathBuf};

macro_rules! directory {
  ($name:ident) => {
    #[derive(Debug)]
    pub struct $name(PathBuf);

    impl Deref for $name {
      type Target = PathBuf;

      fn deref(&self) -> &Self::Target { &self.0 }
    }

    impl <P: AsRef<Path>> From<P> for $name {
      fn from(path: P) -> Self { Self(PathBuf::from(path.as_ref())) }
    }
  };

  ($($name:ident),+ $(,)?) => { $(directory!($name);)+ }
}

directory!(MultiMCDirectory, CurseForgeDirectory, FTBDirectory);

impl Default for MultiMCDirectory {
  fn default() -> Self {
    #[cfg(windows)] let home = std::env::var("USERPROFILE").unwrap_or_default();
    #[cfg(not(windows))] let home = std::env::var("HOME").unwrap_or_default();
    let path = format!("{}/MultiMC/instances", home);

    Self(PathBuf::from(path))
  }
}

impl Default for CurseForgeDirectory {
  fn default() -> Self {
    #[cfg(windows)] let home = std::env::var("USERPROFILE").unwrap_or_default();
    #[cfg(not(windows))] let home = std::env::var("HOME").unwrap_or_default();

    #[cfg(windows)] let path = format!("{}/curseforge/minecraft/Instances", home);
    #[cfg(target_os = "macos")] let path = format!(r"{}/Documents/curseforge/minecraft/Instances", home);
    #[cfg(target_os = "linux")] let path = format!(r"{}/curseforge/minecraft/Instances", home);

    Self(PathBuf::from(path))
  }
}

impl Default for FTBDirectory {
  fn default() -> Self {
    #[cfg(windows)] let home = std::env::var("USERPROFILE").unwrap_or_default();
    #[cfg(not(windows))] let home = std::env::var("HOME").unwrap_or_default();

    #[cfg(windows)] let path = format!("{}/AppData/Local/.ftba/instances", home);
    // TODO MacOS and Linux valid FTB Defaults
    #[cfg(target_os = "macos")] let path = format!(r"{}/Documents/curseforge/minecraft/Instances", home);
    #[cfg(target_os = "linux")] let path = format!(r"{}/curseforge/minecraft/Instances", home);

    Self(PathBuf::from(path))
  }
}

pub struct MultiMCInstance;
pub struct CurseForgeInstance;
pub struct FTBInstance;

impl CurseForgeDirectory {
  fn instances(&self) -> anyhow::Result<Vec<CurseForgeInstance>> {
    let dirs = std::fs::read_dir(self.deref())?;

    Err(anyhow::Error::msg("Unimplemented"))
  }
}
use std::path::{Path, PathBuf};
use std::process::ExitStatus;

macro_rules! directory {
  ($($name:ident),+) => {
    $(#[derive(Debug, Clone)]
    pub struct $name {
      pub path: PathBuf,
    }

    impl Directory for $name {
      fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
          path: path.as_ref().to_path_buf()
        }
      }

      fn path(&self) -> &Path { &self.path }
       fn new_path<P: AsRef<Path>>(&mut self, path: P) {
        self.path = path.as_ref().to_path_buf();
      }
    })+
  };
}

pub trait Directory {
  fn new<P: AsRef<Path>>(path: P) -> Self;
  fn path(&self) -> &Path;
  fn new_path<P: AsRef<Path>>(&mut self, path: P);

  fn exists(&self) -> bool {
    self.path().exists()
  }

  fn to_string(&self) -> String {
    self.path().to_str().unwrap_or_default().to_string()
  }

  fn browse(&mut self) -> native_dialog::Result<()> {
    let mut fd = native_dialog::FileDialog::default();

    if self.path().exists() {
      fd = fd.set_location(self.path());
    }

    let dir = fd.show_open_single_dir()?;

    if let Some(dir) = dir {
      self.new_path(dir);
    }

    Ok(())
  }

  fn open(&self) -> std::io::Result<ExitStatus> {
    open::that(self.path())
  }
}

directory!(MultiMCDirectory, CurseForgeDirectory);

impl Default for MultiMCDirectory {
  fn default() -> Self {
    #[cfg(windows)] let data = std::env::var("APPDATA").unwrap_or_default();
    #[cfg(windows)] let path = format!(r"{}\MultiMC\minecraft\Instances", data);
    #[cfg(not(windows))] let home = std::env::var("HOME").unwrap_or_default();
    #[cfg(not(windows))] let path = format!(r"{}/.local/share/MultiMC/minecraft/Instances", home);

    Self { path: Path::new(&path).into() }
  }
}

impl Default for CurseForgeDirectory {
  fn default() -> Self {
    #[cfg(windows)] let user = std::env::var("USERNAME").unwrap_or_default();
    #[cfg(not(windows))] let user = std::env::var("HOME").unwrap_or_default();
    #[cfg(windows)] let path = format!(r"C:\Users\{}\curseforge\minecraft\Instances", user);
    #[cfg(target_os = "macos")] let path = format!(r"{}/Documents/curseforge/minecraft/Instances", user);
    // FIXME: This is to be updated once CurseForge supports linux
    #[cfg(target_os = "linux")] let path = format!(r"{}/curseforge/minecraft/Instances", user);

    Self { path: Path::new(&path).into() }
  }
}
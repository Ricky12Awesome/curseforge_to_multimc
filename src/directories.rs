use std::path::Path;

macro_rules! directory {
  ($($name:ident),+) => {
    $(#[derive(Debug, Clone)]
    pub struct $name {
      pub path: Box<Path>,
    }

    impl Directory for $name {
      fn path(&self) -> &Path { &self.path }
      fn path_mut(&mut self) -> &mut Path { &mut self.path }
      fn new_path(&mut self, path: Box<Path>) {
        self.path = path;
      }
    })+
  };
}

pub trait Directory {
  fn path(&self) -> &Path;
  fn path_mut(&mut self) -> &mut Path;
  fn new_path(&mut self, path: Box<Path>);

  fn exists(&self) -> bool {
    self.path().exists()
  }

  fn name(&self) -> String {
    self.path().to_str().unwrap_or_default().to_string()
  }

  fn name_if_exists(&self) -> String {
    if self.exists() {
      self.name()
    } else {
      String::from("")
    }
  }

  fn browse(&mut self) {
    let mut fd = native_dialog::FileDialog::default();

    if self.path().exists() {
      fd = fd.set_location(self.path());
    }

    if let Ok(Some(dir)) = fd.show_open_single_dir() {
      self.new_path(dir.as_path().into());
    }
  }
}

directory!(MultiMCDirectory, CurseForgeDirectory);

impl Default for MultiMCDirectory {
  fn default() -> Self {
    #[cfg(windows)] let path = String::from(r"C:\Tools\MultiMC\instances");
    #[cfg(not(windows))] let path = String::from(r"/Tools/MultiMC/instances");

    Self { path: Path::new(&path).into() }
  }
}

impl Default for CurseForgeDirectory {
  fn default() -> Self {
    #[cfg(windows)] let user = std::env::var("USERNAME").unwrap_or_default();
    #[cfg(not(windows))] let user = std::env::var("HOME").unwrap_or_default();
    #[cfg(windows)] let path = format!(r"C:\Users\{}\curseforge\minecraft\Instances", user);
    #[cfg(target_os = "macos")] let path = format!(r"{}/Documents/curseforge/minecraft/Instances", user);
    // FIXME: This is be updated once CurseForge linux
    #[cfg(target_os = "linux")] let path = format!(r"{}/curseforge/minecraft/Instances", user);

    Self { path: Path::new(&path).into() }
  }
}
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use crate::directory::{CurseForgeDirectory, FTBDirectory, MultiMCDirectory};

#[derive(Default, Debug)]
pub struct Settings {
  pub mmc_path: MultiMCDirectory,
  pub cf_path: CurseForgeDirectory,
  pub ftb_path: FTBDirectory,
  // Other Launchers Paths
}

impl Display for Settings {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "mmc = {}", self.mmc_path.display())?;
    writeln!(f, "cf = {}", self.cf_path.display())?;
    writeln!(f, "ftb = {}", self.ftb_path.display())
  }
}

impl Settings {
  pub fn save_to<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
    let mut file = File::create(path)?;

    self.write_to(&mut file)
  }

  pub fn load_from<P: AsRef<Path>>(path: P) -> anyhow::Result<Settings> {
    let mut file = File::open(path)?;

    Self::read_from(&mut file)
  }

  pub fn write_to(&self, writer: &mut impl Write) -> anyhow::Result<()> {
    writer.write(format!("{}", self).as_bytes())?;

    Ok(())
  }

  pub fn read_from(reader: &mut impl Read) -> anyhow::Result<Settings> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    let Self {
      mut mmc_path,
      mut cf_path,
      mut ftb_path
    } = Self::default();

    for str in buf.lines() {
      let (key, value) = str.split_once("=").unwrap_or(("", ""));
      let (key, value) = (key.trim(), value.trim());

      if key.is_empty() {
        continue
      }

      match key {
        "mmc" => mmc_path = MultiMCDirectory::from(value),
        "cf" => cf_path = CurseForgeDirectory::from(value),
        "ftb" => ftb_path = FTBDirectory::from(value),
        _ => ()
      }
    }

    Ok(Settings { mmc_path, cf_path, ftb_path })
  }
}


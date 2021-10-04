use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct Settings {
  pub mmc_path: PathBuf,
  pub cf_path: PathBuf,
  pub ftb_path: PathBuf,
  // Other Launchers Paths
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
    let str = format!(r"
mmc = {}
cf = {}
ftb = {}
", self.mmc_path.display(), self.cf_path.display(), self.ftb_path.display());

    writer.write(str.trim().as_bytes())?;

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

    for (line, str) in buf.lines().enumerate() {
      let (key, value) = str
        .split_once("=")
        .ok_or_else(|| anyhow::Error::msg(format!("Invalid format at line {}", line)))?;

      let (key, value) = (key.trim(), value.trim());

      match key {
        "mmc" => mmc_path = PathBuf::from(value),
        "cf" => cf_path = PathBuf::from(value),
        "ftb" => ftb_path = PathBuf::from(value),
        _ => ()
      }
    }

    Ok(Settings { mmc_path, cf_path, ftb_path })
  }
}


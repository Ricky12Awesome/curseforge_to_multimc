use std::fs::{File, OpenOptions};
use std::path::Path;

use clap::Arg;
use iced::window::Icon;
use serde::{Deserialize, Serialize};

use crate::TITLE;

pub type AnyResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn gen_icon() -> AnyResult<Icon> {
  const SOURCE: &[u8] = include_bytes!("../assets/icon.ico");

  let image = ::image::load_from_memory(SOURCE)?;
  let image = image.to_rgba8();
  let bytes = image.pixels()
    .map(|it| it.0.iter())
    .flatten()
    .map(|it| *it)
    .collect::<Vec<_>>();

  let icon = Icon::from_rgba(bytes, 256, 256)?;

  Ok(icon)
}

pub fn icon() -> std::result::Result<Icon, iced::Error> {
  gen_icon().map_err(|it| iced::Error::WindowCreationFailed(it))
}

// Need this to make CLI work, but will still hide console when ran normally (double clicking, start menu, etc)
pub fn hide_console() {
  #[cfg(windows)] {
    use std::ptr;
    use winapi::um::wincon::GetConsoleWindow;
    use winapi::um::winuser::{ShowWindow, SW_HIDE};

    let window = unsafe { GetConsoleWindow() };
    // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow
    if window != ptr::null_mut() {
      unsafe {
        ShowWindow(window, SW_HIDE);
      }
    }
  }
}

#[derive(Default, Clone)]
pub struct Flags {
  pub settings_path: String,
}

impl Flags {
  pub fn new() -> Self {
    let matches = clap::App::new(TITLE)
      .version(env!("CARGO_PKG_VERSION"))
      .author(env!("CARGO_PKG_AUTHORS"))
      .about("Links CurseForge instances to MultiMC Instances")
      .arg(
        Arg::with_name("settings")
          .long("settings")
          .value_name("FILE")
          .help("Sets custom settings file (this will be written to on change)")
          .takes_value(true)
          .default_value("settings.json")
      )
      .get_matches();

    Self {
      settings_path: matches.value_of("settings").unwrap_or("settings.json").to_string()
    }
  }

  pub fn load_settings(&self) -> ApplicationSettings {
    ApplicationSettings::load_from(&self.settings_path)
  }

  pub fn save_settings(&self, settings: &ApplicationSettings) -> AnyResult<()> {
    settings.save_to(&self.settings_path)
  }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ApplicationSettings {
  pub mmc_directory: Option<String>,
  pub cf_directory: Option<String>,
}

impl ApplicationSettings {
  pub fn load_from<P: AsRef<Path>>(path: P) -> Self {
    match File::open(path) {
      Ok(file) => serde_json::from_reader(&file).unwrap_or_default(),
      Err(_) => Self::default()
    }
  }

  pub fn save_to<P: AsRef<Path>>(&self, path: P) -> AnyResult<()> {
    if path.as_ref().exists() {
      let file = &OpenOptions::new().write(true).open(path)?;

      serde_json::to_writer_pretty(file, self)?;
    } else {
      serde_json::to_writer_pretty(&File::create(path)?, self)?;
    }

    Ok(())
  }
}
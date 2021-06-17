use clap::Arg;
use iced::window::Icon;
use serde::{Deserialize, Serialize};

use crate::{TITLE, NAME};

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
  pub settings_path: Option<String>,
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
          .help("Custom settings path (settings format is TOML)")
          .takes_value(true)
      )
      .get_matches();

    Self {
      settings_path: matches.value_of("settings").map(str::to_string)
    }
  }

  pub fn load_settings(&self) -> ApplicationSettings {
    match &self.settings_path {
      Some(path) => confy::load_path(path),
      None => confy::load(NAME),
    }.unwrap_or_default()
  }

  pub fn save_settings(&self, settings: &ApplicationSettings) -> AnyResult<()> {
    match &self.settings_path {
      Some(path) => confy::store_path(path, settings),
      None => confy::store(NAME, settings)
    }?;

    Ok(())
  }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ApplicationSettings {
  pub mmc_directory: Option<String>,
  pub cf_directory: Option<String>,
}

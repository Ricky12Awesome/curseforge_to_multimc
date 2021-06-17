// #![windows_subsystem = "windows"] // Doesn't work for CLI + GUI Applications

use std::fs::{File, OpenOptions};
use std::marker::PhantomData;
use std::path::Path;

use clap::Arg;
use iced::*;
use iced::window::Icon;
use serde::{Deserialize, Serialize};

use crate::directories::{CurseForgeDirectory, Directory, MultiMCDirectory};
use crate::modpack::CFModPack;
use iced_native::Event;

mod directories;
mod link;
mod modpack;

const TITLE: &'static str = "CurseForge to MultiMC";
const GITHUB_URL: &'static str = "https://github.com/Ricky12Awesome/curseforge_to_multimc";
const ERR_COLOR: Color = Color { r: 0.8, g: 0.0, b: 0.0, a: 1.0 };
const OK_COLOR: Color = Color { r: 0.0, g: 0.8, b: 0.0, a: 1.0 };
const IMPORTANT_SIZE: u16 = 24;
const IMPORTANT_COLOR: Color = Color { r: 0.0, g: 0.0, b: 0.8, a: 1.0 };

// #[macro_export]
// macro_rules! make_multi_mut_ref {
//   ($v:expr, $c:ty) => { unsafe { &mut *($v as *mut $c) } };
// }
//
// #[macro_export]
// macro_rules! make_multi_ref {
//   ($v:expr, $c:ty) => { unsafe { &*($v as *const $c) } };
// }

type AnyResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

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

fn icon() -> std::result::Result<Icon, Error> {
  gen_icon().map_err(|it| iced::Error::WindowCreationFailed(it))
}

// Need this to make CLI work, but will still hide console when ran normally (double clicking, start menu, etc)
#[cfg(windows)]
fn hide_console() {
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

fn main() -> Result {
  let flags = Flags::new();

  #[cfg(windows)] hide_console();

  <CurseForgeToMultiMC as Application>::run(Settings {
    flags,
    window: window::Settings {
      icon: Some(icon()?),
      size: (975, 650),
      min_size: Some((975, 600)),
      ..Default::default()
    },
    ..Default::default()
  })
}

#[derive(Default, Clone)]
struct Flags {
  settings_path: String,
}

impl Flags {
  fn new() -> Self {
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

  fn load_settings(&self) -> ApplicationSettings {
    ApplicationSettings::load_from(&self.settings_path)
  }

  fn save_settings(&self, settings: &ApplicationSettings) -> AnyResult<()> {
    settings.save_to(&self.settings_path)
  }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct ApplicationSettings {
  mmc_directory: Option<String>,
  cf_directory: Option<String>,
}

impl ApplicationSettings {
  fn load_from<P: AsRef<Path>>(path: P) -> Self {
    match File::open(path) {
      Ok(file) => serde_json::from_reader(&file).unwrap_or_default(),
      Err(_) => Self::default()
    }
  }

  fn save_to<P: AsRef<Path>>(&self, path: P) -> AnyResult<()> {
    if path.as_ref().exists() {
      let file = &OpenOptions::new().write(true).open(path)?;

      serde_json::to_writer_pretty(file, self)?;
    } else {
      serde_json::to_writer_pretty(&File::create(path)?, self)?;
    }

    Ok(())
  }
}

#[derive(Default)]
struct CurseForgeToMultiMC<'a> {
  mmc_d: MultiMCDirectory,
  cf_d: CurseForgeDirectory,
  mmc_ti_d_state: text_input::State,
  mmc_browse_state: button::State,
  cf_ti_d_state: text_input::State,
  cf_browse_state: button::State,
  pick_cf_mp: pick_list::State<CFModPack>,
  link_btn_state: button::State,
  unlink_btn_state: button::State,
  open_btn_state: button::State,
  github_btn_state: button::State,
  save_btn: button::State,
  selected_cf_mp: Option<CFModPack>,
  info: Option<(Color, String)>,
  flags: Flags,
  settings: ApplicationSettings,
  should_exit: bool,
  _data: PhantomData<&'a ()>, // Just in case I need it later
}

#[derive(Debug, Clone)]
enum Message {
  MMCDirectoryChange(String),
  CFDirectoryChange(String),
  MMCBrowse,
  CFBrowse,
  CFMPPicked(CFModPack),
  Link,
  Unlink,
  Open,
  OpenGithub,
  Save,
}

impl<'a> CurseForgeToMultiMC<'a> {}

impl<'a> Application for CurseForgeToMultiMC<'a> {
  type Executor = iced::executor::Default;
  type Message = Message;
  type Flags = Flags;

  fn new(flags: Flags) -> (Self, Command<Message>) {
    let settings = flags.load_settings();
    let mmc_d = settings.mmc_directory.as_ref()
      .map(MultiMCDirectory::new)
      .unwrap_or_default();

    let cf_d = settings.cf_directory.as_ref()
      .map(CurseForgeDirectory::new)
      .unwrap_or_default();


    (Self { mmc_d, cf_d, flags, settings, ..Self::default() }, Command::none())
  }

  fn title(&self) -> String {
    String::from(TITLE)
  }

  fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
    match message {
      Message::MMCDirectoryChange(dir) => {
        self.mmc_d.new_path(&dir);
        self.settings.mmc_directory = Some(dir);
      }
      Message::CFDirectoryChange(dir) => {
        self.cf_d.new_path(&dir);
        self.settings.cf_directory = Some(dir);
      }
      Message::MMCBrowse => {
        self.mmc_d.browse();
        self.settings.mmc_directory = Some(self.mmc_d.to_string());
      }
      Message::CFBrowse => {
        self.cf_d.browse();
        self.settings.cf_directory = Some(self.cf_d.to_string());
      }
      Message::CFMPPicked(new) => {
        self.selected_cf_mp = Some(new);
        self.info = None;
      }
      Message::Link => {
        if let Some(selected) = &self.selected_cf_mp {
          let result = crate::link::link(
            self.mmc_d.clone(),
            self.cf_d.clone(),
            selected.clone(),
          );

          self.info = result.as_ref().ok().map(|_| (OK_COLOR, String::from("Linked")));

          if let None = self.info {
            self.info = result.as_ref().err().map(|it| (ERR_COLOR, it.to_string()))
          }
        }
      }
      Message::Unlink => {
        if let Some(selected) = &self.selected_cf_mp {
          let result = crate::link::unlink(
            self.mmc_d.clone(),
            selected.clone(),
          );

          self.info = result.as_ref().ok().map(|_| (OK_COLOR, String::from("Unlinked")));

          if let None = self.info {
            self.info = result.as_ref().err().map(|it| (ERR_COLOR, it.to_string()))
          }
        }
      }
      Message::Open => {
        if let Some(selected) = &self.selected_cf_mp {
          if let Some(dir) = &selected.dir {
            let result = open::that(dir);

            self.info = result.err().map(|it| (ERR_COLOR, it.to_string()))
          }
        }
      }
      Message::OpenGithub => {
        let result = open::that(GITHUB_URL);

        self.info = result.err().map(|it| (ERR_COLOR, it.to_string()))
      }
      Message::Save => {
        self.flags.save_settings(&self.settings).unwrap();

        self.should_exit = true;
      }
    }

    Command::none()
  }

  fn subscription(&self) -> Subscription<Message> {
    iced_native::subscription::events_with(|event, _| {
      match event {
        Event::Window(iced_native::window::Event::CloseRequested) => {
          println!("Yes");

          Some(Message::Save)
        }
        _ => None
      }
    })
  }

  fn view(&mut self) -> Element<Message> {
    Column::new()
      .padding(20)
      .spacing(8)
      .align_items(Align::Center)
      .push(
        Row::new()
          .push(Text::new("MultiMC Directory: "))
          .push(
            TextInput::new(
              &mut self.mmc_ti_d_state, "", &self.mmc_d.to_string(),
              Message::MMCDirectoryChange,
            )
          )
          .push(
            Button::new(&mut self.mmc_browse_state, Text::new("Browse"))
              .on_press(Message::MMCBrowse)
          )
      )
      .push(
        Row::new()
          .push(Text::new("CurseForge Directory: "))
          .push(
            TextInput::new(
              &mut self.cf_ti_d_state, "", &self.cf_d.to_string(),
              Message::CFDirectoryChange,
            )
          )
          .push(
            Button::new(&mut self.cf_browse_state, Text::new("Browse"))
              .on_press(Message::CFBrowse)
          )
      )
      .push(
        PickList::new(
          &mut self.pick_cf_mp,
          CFModPack::list(self.cf_d.clone(), &mut self.selected_cf_mp),
          self.selected_cf_mp.clone(),
          Message::CFMPPicked,
        ).width(Length::Fill)
      )
      .push(Space::with_height(Length::Fill))
      .push(Text::new("This is a simple utility to help link CurseForge instances to MultiMC instances").size(IMPORTANT_SIZE))
      .push(Space::with_height(Length::Fill))
      .push(
        Text::new("Icons can't be detected, there's no way to get them from the manifest")
          .size(IMPORTANT_SIZE)
          .color(IMPORTANT_COLOR)
      )
      .push(
        Text::new("Fabric detection only works for modpacks that support it ")
          .size(IMPORTANT_SIZE)
          .color(IMPORTANT_COLOR)
      )
      .push(Space::with_height(Length::Fill))
      .push(
        Text::new("Example: ")
          .size(IMPORTANT_SIZE)
          .color(IMPORTANT_COLOR)
      )
      .push(
        Text::new("\"All Of Fabric 3\" doesn't get detected as fabric cause CurseForge still thinks it's a forge modpack")
          .size(IMPORTANT_SIZE)
          .color(IMPORTANT_COLOR)
      )
      .push(
        Text::new("\"All Of Fabric 4\" does get detected as a fabric modpack")
          .size(IMPORTANT_SIZE)
          .color(IMPORTANT_COLOR)
      )
      .push(Space::new(Length::Fill, Length::Fill))
      .push(
        Button::new(
          &mut self.github_btn_state,
          Text::new("Github"),
        ).on_press(Message::OpenGithub)
      )
      .push(
        match &self.selected_cf_mp {
          Some(mp) => Button::new(
            &mut self.link_btn_state,
            Text::new(format!("Link ({})", mp)),
          ).on_press(Message::Link),
          None => Button::new(
            &mut self.link_btn_state,
            Text::new("Link (None)"),
          )
        }
      )
      .push(
        match &self.selected_cf_mp {
          Some(mp) => Button::new(
            &mut self.unlink_btn_state,
            Text::new(format!("Unlink ({})", mp)),
          ).on_press(Message::Unlink),
          None => Button::new(
            &mut self.unlink_btn_state,
            Text::new("Unlink (None)"),
          )
        }
      )
      .push(
        match &self.selected_cf_mp {
          Some(mp) => Button::new(
            &mut self.open_btn_state,
            Text::new(format!("Open ({})", mp)),
          ).on_press(Message::Open),
          None => Button::new(
            &mut self.open_btn_state,
            Text::new("Open (None)"),
          )
        }
      )
      .push::<Element<Message>>(
        match &self.info {
          Some((color, err)) => {
            Text::new(err)
              .size(IMPORTANT_SIZE)
              .color(*color)
              .into()
          }
          _ => Space::with_height(Length::Units(0)).into(),
        }
      )
      .into()
  }

  fn should_exit(&self) -> bool {
    self.should_exit
  }
}

// #![windows_subsystem = "windows"] // Doesn't work for CLI + GUI Applications

use iced::*;
use iced_native::{Element, Event};

use crate::directories::{CurseForgeDirectory, Directory, MultiMCDirectory};
use crate::ext::ButtonExt;
use crate::misc::{ApplicationSettings, Flags, hide_console, icon};
use crate::modpack::ModPack;

mod directories;
mod ext;
mod link;
mod misc;
mod modpack;

const NAME: &'static str = env!("CARGO_PKG_NAME");
const TITLE: &'static str = "CurseForge to MultiMC";
const GITHUB_URL: &'static str = env!("CARGO_PKG_REPOSITORY");
const ERR_COLOR: Color = Color { r: 0.8, g: 0.0, b: 0.0, a: 1.0 };
const OK_COLOR: Color = Color { r: 0.0, g: 0.8, b: 0.0, a: 1.0 };
const IMPORTANT_SIZE: u16 = 24;

macro_rules! set_info_if_err {
  ($info:expr, $value:expr) => {
    $info = $value.err().map(|it| (ERR_COLOR, it.to_string()))
  };
}

fn main() -> Result {
  let flags = Flags::new();

  hide_console();

  <CurseForgeToMultiMC as Application>::run(Settings {
    flags,
    exit_on_close_request: false,
    window: window::Settings {
      icon: Some(icon()?),
      size: (750, 700),
      min_size: Some((650, 425)),
      ..Default::default()
    },
    ..Default::default()
  })
}

#[derive(Default)]
struct CurseForgeToMultiMC {
  mmc_d: MultiMCDirectory,
  cf_d: CurseForgeDirectory,
  mmc_ti_d_state: text_input::State,
  mmc_browse_state: button::State,
  mmc_open_state: button::State,
  cf_ti_d_state: text_input::State,
  cf_browse_state: button::State,
  cf_open_state: button::State,
  pick_mp_state: pick_list::State<ModPack>,
  link_btn_state: button::State,
  unlink_btn_state: button::State,
  open_cf_btn_state: button::State,
  open_mmc_btn_state: button::State,
  github_btn_state: button::State,
  selected_mp: Option<ModPack>,
  info: Option<(Color, String)>,
  flags: Flags,
  settings: ApplicationSettings,
  should_exit: bool,
}

#[derive(Debug, Clone)]
enum Message {
  MMCDirectoryChange(String),
  CFDirectoryChange(String),
  MMCBrowse,
  MMCOpen,
  CFBrowse,
  CFOpen,
  CFMPPicked(ModPack),
  Link,
  Unlink,
  OpenSelectedCF,
  OpenSelectedMMC,
  OpenGithub,
  Save,
}

impl Application for CurseForgeToMultiMC {
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

  fn update(&mut self, message: Message) -> Command<Message> {
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
        set_info_if_err!(self.info, self.mmc_d.browse());
        self.settings.mmc_directory = Some(self.mmc_d.to_string());
      }
      Message::MMCOpen => {
        set_info_if_err!(self.info, self.mmc_d.open());
      }
      Message::CFBrowse => {
        set_info_if_err!(self.info, self.mmc_d.browse());
        self.settings.cf_directory = Some(self.cf_d.to_string());
      }
      Message::CFOpen => {
        set_info_if_err!(self.info, self.cf_d.open());
      }
      Message::CFMPPicked(new) => {
        self.selected_mp = Some(new);
        self.info = None;
      }
      Message::Link => {
        if let Some(selected) = &self.selected_mp {
          let result = crate::link::link(
            self.mmc_d.clone(),
            self.cf_d.clone(),
            selected.clone(),
          );

          self.info = result.as_ref().ok().map(|_| (OK_COLOR, String::from("Linked")));

          if let None = self.info {
            set_info_if_err!(self.info, result.as_ref());
          }
        }
      }
      Message::Unlink => {
        if let Some(selected) = &self.selected_mp {
          let result = crate::link::unlink(
            self.mmc_d.clone(),
            selected.clone(),
          );

          self.info = result.as_ref().ok().map(|_| (OK_COLOR, String::from("Unlinked")));

          if let None = self.info {
            set_info_if_err!(self.info, result.as_ref());
          }
        }
      }
      Message::OpenSelectedCF => {
        if let Some(selected) = &self.selected_mp {
          if let Some(dir) = &selected.cf_dir {
            let result = open::that(dir);

            set_info_if_err!(self.info, result.as_ref());
          }
        }
      }
      Message::OpenSelectedMMC => {
        if let Some(selected) = &self.selected_mp {
          if let Some(dir) = &selected.mmc_path(&self.mmc_d) {
            let result = open::that(dir);

            set_info_if_err!(self.info, result.as_ref());
          }
        }
      }
      Message::OpenGithub => {
        let result = open::that(GITHUB_URL);

        set_info_if_err!(self.info, result.as_ref());
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
        Event::Window(iced_native::window::Event::CloseRequested) => Some(Message::Save),
        _ => None
      }
    })
  }

  fn view(&mut self) -> Element<Message, Renderer> {
    let is_linked = self.selected_mp.clone().unwrap_or_default().is_linked(&self.mmc_d);

    Column::new()
      .padding(20)
      .spacing(8)
      .align_items(Alignment::Center)
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
          .push(
            Button::new(&mut self.mmc_open_state, Text::new("Open"))
              .on_press(Message::MMCOpen)
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
          .push(
            Button::new(&mut self.cf_open_state, Text::new("Open"))
              .on_press(Message::CFOpen)
          )
      )
      .push(
        PickList::new(
          &mut self.pick_mp_state,
          ModPack::list(self.cf_d.clone(), &mut self.selected_mp),
          self.selected_mp.clone(),
          Message::CFMPPicked,
        ).width(Length::Fill)
      )
      .push(
        Row::new()
          .push(
            Button::new(
              &mut self.open_cf_btn_state,
              Text::new("Open CurseForge Instance"),
            ).on_press_if(Message::OpenSelectedCF, self.selected_mp.is_some())
          )
          .push(Space::with_width(Length::Units(12)))
          .push(
            Button::new(
              &mut self.open_mmc_btn_state,
              Text::new("Open MultiMC Instance"),
            ).on_press_if(Message::OpenSelectedMMC, is_linked)
          )
      )
      .push(
        Row::new()
          .push(
            Button::new(
              &mut self.link_btn_state,
              Text::new("Link"),
            ).on_press_if(Message::Link, self.selected_mp.is_some() && !is_linked)
          )
          .push(Space::with_width(Length::Units(12)))
          .push(
            Button::new(
              &mut self.unlink_btn_state,
              Text::new("Unlink"),
            ).on_press_if(Message::Unlink, is_linked)
          )
      )
      .push(
        Button::new(
          &mut self.github_btn_state,
          Text::new("Github"),
        ).on_press(Message::OpenGithub)
      )
      .push(Space::new(Length::Fill, Length::Fill))
      .push(
        Text::new("Application may be slow to respond, I do not know how to fix this, blame windows I guess")
          .size(28)
      )
      .push(Space::new(Length::Fill, Length::Fill))
      .push::<Element<Message, Renderer>>(
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

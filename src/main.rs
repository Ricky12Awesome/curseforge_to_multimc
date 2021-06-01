#![windows_subsystem = "windows"]
use std::marker::PhantomData;
use std::path::Path;

use iced::*;

use crate::modpack::{CFModPack, CFModPackCache};
use crate::util::{CurseForgeDirectory, Directory, MultiMCDirectory};

mod util;
mod link;
mod modpack;

const TITLE: &'static str = "CurseForge to MultiMC";
const IMPORTANT_SIZE: u16 = 35;

#[macro_export]
macro_rules! make_multi_mut_ref {
  ($v:expr, $c:ty) => { unsafe { &mut *($v as *mut $c) } };
}

#[macro_export]
macro_rules! make_multi_ref {
  ($v:expr, $c:ty) => { unsafe { &*($v as *const $c) } };
}

fn main() -> iced::Result {
  <CurseForgeToMultiMC as Sandbox>::run(Settings {
    window: window::Settings {
      size: (1280, 720),
      min_size: Some((640, 480)),
      ..Default::default()
    },
    ..Default::default()
  })
}

#[derive(Default)]
pub struct CurseForgeToMultiMC<'a> {
  mmc_d: MultiMCDirectory,
  cf_d: CurseForgeDirectory,
  mmc_ti_d_state: text_input::State,
  mmc_browse_state: button::State,
  cf_ti_d_state: text_input::State,
  cf_browse_state: button::State,
  pick_cf_mp: pick_list::State<CFModPack>,
  link_btn_state: button::State,
  open_btn_state: button::State,
  pick_cf_mp_cache: CFModPackCache,
  selected_cf_mp: Option<CFModPack>,
  info: Option<(bool, String)>,
  _data: PhantomData<&'a ()>, // Just in case I need it later
}

#[derive(Debug, Clone)]
pub enum Message {
  MMCDirectoryChange(String),
  CFDirectoryChange(String),
  MMCBrowse,
  CFBrowse,
  CFMPPicked(CFModPack),
  Link,
  Open
}

impl<'a> CurseForgeToMultiMC<'a> {
  fn mmc_d(&mut self) -> Element<Message> {
    Row::new()
      .push(Text::new("MultiMC Directory: "))
      .push(
        TextInput::new(
          &mut self.mmc_ti_d_state, "", &self.mmc_d.name(),
          Message::MMCDirectoryChange,
        )
      )
      .push(
        Button::new(&mut self.mmc_browse_state, Text::new("Browse"))
          .on_press(Message::MMCBrowse)
      )
      .into()
  }

  fn cf_d(&mut self) -> Element<Message> {
    Row::new()
      .push(Text::new("CurseForge Directory: "))
      .push(
        TextInput::new(
          &mut self.cf_ti_d_state, "", &self.cf_d.name(),
          Message::CFDirectoryChange,
        )
      )
      .push(
        Button::new(&mut self.cf_browse_state, Text::new("Browse"))
          .on_press(Message::CFBrowse)
      )
      .into()
  }

  fn show_result(&self) -> Element<Message> {
    match &self.info {
      Some((false, _)) => {
        Text::new("Link Successful")
          .size(IMPORTANT_SIZE)
          .color(Color::new(0.0, 0.75, 0.0, 1.0))
          .into()
      }
      Some((true, err)) => {
        println!("[Link Result Error]: {:?}", err);
        Text::new(err)
          .size(IMPORTANT_SIZE)
          .color(Color::new(0.75, 0.0, 0.0, 1.0))
          .into()
      }
      _ => Space::with_height(Length::Units(0)).into(),
    }
  }
}

impl<'a> Sandbox for CurseForgeToMultiMC<'a> {
  type Message = Message;

  fn new() -> Self {
    Self::default()
  }

  fn title(&self) -> String {
    String::from(TITLE)
  }

  fn update(&mut self, message: Message) {
    match message {
      Message::MMCDirectoryChange(dir) => {
        self.mmc_d.new_path(Path::new(&dir).into())
      }
      Message::CFDirectoryChange(dir) => {
        self.cf_d.new_path(Path::new(&dir).into())
      }
      Message::MMCBrowse => {
        self.mmc_d.browse();
      }
      Message::CFBrowse => {
        self.cf_d.browse();
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

          self.info = result.err().map(|it| (true, it.to_string()))
        }
      }
      Message::Open => {
        if let Some(selected) = &self.selected_cf_mp {
          if let Some(dir) = &selected.dir {
            let result = open::that(dir);

            self.info = result.err().map(|it| (true, it.to_string()))
          }
        }
      }
    }
  }

  fn view(&mut self) -> Element<Message> {
    Column::new()
      .padding(20)
      .spacing(8)
      .align_items(Align::Center)
      .push(make_multi_mut_ref!(self, Self).mmc_d())
      .push(make_multi_mut_ref!(self, Self).cf_d())
      .push(
        PickList::new(
          make_multi_mut_ref!(&mut self.pick_cf_mp, pick_list::State<_>),
          CFModPack::list(make_multi_mut_ref!(self, Self)),
          self.selected_cf_mp.clone(),
          Message::CFMPPicked,
        ).width(Length::Fill)
      )
      .push(Space::with_height(Length::Fill))
      .push(Text::new("This is a simple utility to help link CurseForge modpacks to MultiMC profiles").size(IMPORTANT_SIZE))
      .push(
        Text::new("IMPORTANT: ")
          .size(IMPORTANT_SIZE)
          .color(Color::new(0.75, 0.0, 0.0, 1.0))
      )
      .push(
        Text::new("Make sure you double check the versions in MultiMC")
          .size(IMPORTANT_SIZE)
          .color(Color::new(0.75, 0.0, 0.0, 1.0))
      )
      .push(
        Text::new("If this is a fabric pack, you need to change it from forge to fabric in MultiMC")
          .size(IMPORTANT_SIZE)
          .color(Color::new(0.75, 0.0, 0.0, 1.0))
      )
      .push(Space::new(Length::Fill, Length::Fill))
      .push(
        match &self.selected_cf_mp {
          Some(mp) => Button::new(
            make_multi_mut_ref!(&mut self.link_btn_state, button::State),
            Text::new(format!("Link ({})", mp)),
          ).on_press(Message::Link),
          None => Button::new(
            make_multi_mut_ref!(&mut self.link_btn_state, button::State),
            Text::new("Link (None)"),
          )
        }
      )
      .push(
        match &self.selected_cf_mp {
          Some(mp) => Button::new(
            make_multi_mut_ref!(&mut self.open_btn_state, button::State),
            Text::new(format!("Open ({})", mp)),
          ).on_press(Message::Open),
          None => Button::new(
            make_multi_mut_ref!(&mut self.open_btn_state, button::State),
            Text::new("Open (None)"),
          )
        }
      )
      .push(make_multi_ref!(self, Self).show_result())
      .into()
  }
}

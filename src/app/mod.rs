use iced::*;

use crate::flags::Flags;
use crate::TITLE;

mod icon;

use self::icon::get_icon;

pub fn run_app() -> anyhow::Result<()> {
  let settings = Settings {
    window: window::Settings {
      icon: get_icon()?.into(),
      ..Default::default()
    },
    ..Default::default()
  };

  <App as Application>::run(settings)?;

  Ok(())
}

struct App;

#[derive(Debug)]
enum Message {
  
}

impl Application for App {
  type Executor = executor::Default;
  type Message = Message;
  type Flags = Flags;

  fn new(flags: Flags) -> (Self, Command<Message>) {
    (App, Command::none())
  }

  fn title(&self) -> String {
    TITLE.to_string()
  }

  fn update(&mut self, message: Message, clipboard: &mut Clipboard) -> Command<Message> {
    Command::none()
  }

  fn view(&mut self) -> Element<'_, Message> {
    Row::new().into()
  }
}
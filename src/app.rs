use iced::*;

use crate::event::Event;
use crate::flags::Flags;
use crate::TITLE;

struct App;

impl Application for App {
  type Executor = executor::Default;
  type Message = Event;
  type Flags = Flags;

  fn new(flags: Flags) -> (Self, Command<Event>) {
    (App, Command::none())
  }

  fn title(&self) -> String {
    TITLE.to_string()
  }

  fn update(&mut self, message: Event, clipboard: &mut Clipboard) -> Command<Event> {
    Command::none()
  }

  fn view(&mut self) -> Element<'_, Event> {
    Row::new().into()
  }
}
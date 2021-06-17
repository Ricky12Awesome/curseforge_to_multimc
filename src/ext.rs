pub trait ButtonExt<'a, Message> {
  fn on_press_if(self, message: Message, check: bool) -> Self;
}

impl<'a, Message: Clone> ButtonExt<'a, Message> for iced::Button<'a, Message> {
  fn on_press_if(self, message: Message, check: bool) -> Self {
    if check {
      self.on_press(message)
    } else {
      self
    }
  }
}
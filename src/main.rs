use crate::flags::Flags;
use crate::settings::Settings;

mod app;
mod cmd;
mod event;
mod flags;
mod settings;

const TITLE: &'static str = "Link to MultiMC";

fn main() -> anyhow::Result<()> {
  let path = "./test.txt";
  let flags = Flags::default();
  let settings = Settings::load_from(path)?;

  settings.save_to(path)?;

  Ok(())
}


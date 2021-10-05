mod app;
mod cli;
mod flags;
mod instance;
mod link;
mod settings;

const TITLE: &'static str = "Link to MultiMC";

fn main() -> anyhow::Result<()> {
  println!("{:?}", crate::settings::Settings::default());
  // self::app::run_app()?;

  Ok(())
}

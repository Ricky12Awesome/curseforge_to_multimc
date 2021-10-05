mod app;
mod cli;
mod directory;
mod flags;
mod settings;

const TITLE: &'static str = "Link to MultiMC";

fn main() -> anyhow::Result<()> {

  self::app::run_app()?;

  Ok(())
}


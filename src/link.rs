use crate::instance::{CurseForgeInstance, FTBInstance, MultiMCDirectory, MultiMCInstance};

pub enum Instance {
  CF(CurseForgeInstance),
  FTB(FTBInstance),
}

pub fn link(mmc: MultiMCDirectory, dst: Instance) -> anyhow::Result<()> {
  match dst {
    Instance::CF(dst) => Err(anyhow::Error::msg("[Link-CF] Unimplemented")),
    Instance::FTB(dst) => Err(anyhow::Error::msg("[Link-FTB] Unimplemented")),
  }
}

pub fn unlink(src: MultiMCDirectory, dst: Instance) -> anyhow::Result<()> {
  match dst {
    Instance::CF(dst) => Err(anyhow::Error::msg("[Unlink-CF] Unimplemented")),
    Instance::FTB(dst) => Err(anyhow::Error::msg("[Unlink-FTB] Unimplemented")),
  }
}

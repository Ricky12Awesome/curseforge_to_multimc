// Overkill way to handle icons lol

#[cfg(windows)]
mod win_impl {
  use iced_winit::winit::platform::windows::IconExtWindows;

  type IcedIcon = iced::window::Icon;
  type WInitIcon = iced_winit::winit::window::Icon;

  pub fn get_icon() -> anyhow::Result<IcedIcon> {
    let icon = WInitIcon::from_resource(1, None)?;

    let icon = unsafe {
      std::mem::transmute::<WInitIcon, IcedIcon>(icon)
    };

    Ok(icon)
  }
}

#[cfg(not(windows))]
mod not_win_impl {
  use ced::window::Icon;

  const BYTES: &[u8] = include_bytes!("../../assets/raw_icon.bin");

  fn as_n_bytes<const N: usize>(data: &[u8]) -> [u8; N] {
    unsafe {
      *(data.as_ptr() as *const [u8; N])
    }
  }

  pub fn get_icon() -> anyhow::Result<Icon> {
    let width = u32::from_be_bytes(as_n_bytes(&BYTES[0..4]));
    let height = u32::from_be_bytes(as_n_bytes(&BYTES[4..8]));
    let pixels = BYTES[8..].to_vec();
    let icon = Icon::from_rgba(pixels, width, height)?;

    Ok(icon)
  }
}

pub fn get_icon() -> anyhow::Result<iced::window::Icon> {
  #[cfg(windows)] {
    win_impl::get_icon()
  }

  #[cfg(not(windows))] {
    not_win_impl::get_icon()
  }
}
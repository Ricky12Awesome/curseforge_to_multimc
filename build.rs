fn icon_to_raw() -> std::result::Result<(), Box<dyn std::error::Error>> {
  const SOURCE: &[u8] = include_bytes!("./assets/icon.ico");

  let out_dir = std::env::var("OUT_DIR")?;
  let image = ::image::load_from_memory(SOURCE)?;
  let image = image.to_rgba8();
  let pixels = image.pixels()
    .map(|it| it.0.iter())
    .flatten()
    .map(|it| *it)
    .collect::<Vec<_>>();

  let code = format!(r"
    struct RawIcon;

    impl RawIcon {{
      const WIDTH: u32 = {};
      const HEIGHT: u32 = {};
      const PIXELS: &'static [u8] = &{:?};
    }}
  ", image.width(), image.height(), pixels);

  let dir = format!("{}/assets", out_dir);
  let file = format!("{}/raw_icon.rs", dir);

  std::fs::create_dir_all(dir)?;
  std::fs::write(file, code)?;

  Ok(())
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
  icon_to_raw()?;

  #[cfg(windows)] {
    use winres::WindowsResource;

    WindowsResource::new()
      .set_icon("assets/icon.ico")
      .set_language(winapi::um::winnt::MAKELANGID(
        winapi::um::winnt::LANG_ENGLISH,
        winapi::um::winnt::SUBLANG_ENGLISH_US,
      ))
      .set_manifest(r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
<trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
        <requestedPrivileges>
            <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
        </requestedPrivileges>
    </security>
</trustInfo>
</assembly>
"#)
      .compile()?;
  }

  Ok(())
}
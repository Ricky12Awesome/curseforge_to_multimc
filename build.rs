fn icon_to_raw() -> std::result::Result<(), Box<dyn std::error::Error>> {
  const SRC: &str = "assets/raw_icon.bin";
  const ICON: &[u8] = include_bytes!("assets/icon.ico");

  if std::path::Path::new(SRC).exists() {
    return Ok(());
  }

  let image = ::image::load_from_memory(ICON)?;
  let image = image.to_rgba8();
  let pixels = image.pixels()
    .map(|it| it.0.iter())
    .flatten()
    .map(|it| *it)
    .collect::<Vec<_>>();

  let mut data = Vec::<u8>::with_capacity(pixels.len() + 16);

  data.extend_from_slice(&image.width().to_be_bytes());
  data.extend_from_slice(&image.height().to_be_bytes());
  data.extend_from_slice(&pixels);

  std::fs::write(SRC, data)?;

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
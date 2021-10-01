use std::io::Write;
use std::mem::size_of;

fn icon_to_raw() -> std::result::Result<(), Box<dyn std::error::Error>> {
  const SOURCE: &[u8] = include_bytes!("./assets/icon.ico");

  let out_dir = std::env::var("OUT_DIR")?;
  let image = ::image::load_from_memory(SOURCE)?;
  let image = image.to_rgba8();
  let bytes = image.pixels()
    .map(|it| it.0.iter())
    .flatten()
    .map(|it| *it)
    .collect::<Vec<_>>();

  let mut raw_bytes = Vec::with_capacity(size_of::<u32>() * 2 + bytes.len());

  raw_bytes.write(&image.width().to_le_bytes())?;
  raw_bytes.write(&image.height().to_le_bytes())?;
  raw_bytes.write(&bytes)?;

  let dir = format!("{}/assets", out_dir);
  let file = format!("{}/icon.bin", dir);

  std::fs::create_dir_all(dir)?;
  std::fs::write(file, raw_bytes)?;

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
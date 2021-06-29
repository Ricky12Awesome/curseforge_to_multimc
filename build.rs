use std::io;

fn main() -> io::Result<()> {
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
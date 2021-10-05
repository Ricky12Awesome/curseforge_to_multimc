use std::ops::Deref;
use std::path::{Path, PathBuf};

macro_rules! directory {
  ($name:ident) => {
    #[derive(Default, Debug)]
    pub struct $name(PathBuf);

    impl Deref for $name {
      type Target = PathBuf;

      fn deref(&self) -> &Self::Target { &self.0 }
    }

    impl <P: AsRef<Path>> From<P> for $name {
      fn from(path: P) -> Self { Self(PathBuf::from(path.as_ref())) }
    }
  };

  ($($name:ident),+ $(,)?) => { $(directory!($name);)+ }
}

directory!(MultiMCDirectory, CurseForgeDirectory, FTBDirectory);

fn e() {

}
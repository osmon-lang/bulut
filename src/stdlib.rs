use crate::github::schemes::release::Release;
use crate::github::GitHub;

const _STATIC_STD_FILES: &'static [&'static str] = &["libhavo.h", "libhavo.so", "library.zip"];

struct Stdlib {
    github: GitHub,
    version: String,
    files: Vec<String>,
}

pub enum StdlibError {
    NoVersions,
}

impl Stdlib {
    pub fn new() -> Self {
        Self {
            github: GitHub::new("osmon-lang", "havo"),
            version: String::new(),
            files: Vec::new(),
        }
    }

    pub fn fetch_version(&self, v: String) {
        let latest = self.github.releases().unwrap();
    }

    pub fn fetch_latest(&self) -> Result<Release, StdlibError> {
        let latest = self.github.releases().unwrap();

        if latest.len() == 0 {
            return Err(StdlibError::NoVersions);
        }

        Ok(self.github.release(latest.get(0).unwrap()).unwrap())
    }
}

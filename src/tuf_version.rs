use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    version: i32,
}

impl Version {
    const ANY_VERSION: i32 = -1;

    pub fn new() -> Self {
        Version {
            version: Self::ANY_VERSION,
        }
    }

    pub fn from_int(v: i32) -> Self {
        Version { version: v }
    }

    pub fn version(&self) -> i32 {
        self.version
    }

    pub fn role_file_name(&self, role: &str) -> String {
        format!("{}_v{}.json", role, self.version)
    }

    pub fn is_latest(&self) -> bool {
        self.version == Self::ANY_VERSION
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.version)
    }
}

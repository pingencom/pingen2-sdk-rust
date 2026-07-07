use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrintMode {
    Simplex,
    Duplex,
}

impl PrintMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Simplex => "simplex",
            Self::Duplex => "duplex",
        }
    }
}

impl fmt::Display for PrintMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

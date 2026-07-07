use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupingType {
    Merge,
    Zip,
}

impl GroupingType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Merge => "merge",
            Self::Zip => "zip",
        }
    }
}

impl fmt::Display for GroupingType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

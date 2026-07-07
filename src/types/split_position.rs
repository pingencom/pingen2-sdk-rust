use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitPosition {
    FirstPage,
    LastPage,
}

impl SplitPosition {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FirstPage => "first_page",
            Self::LastPage => "last_page",
        }
    }
}

impl fmt::Display for SplitPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

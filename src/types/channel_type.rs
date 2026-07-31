use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelType {
    Post,
    Email,
    Ebill,
}

impl ChannelType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Post => "post",
            Self::Email => "email",
            Self::Ebill => "ebill",
        }
    }
}

impl fmt::Display for ChannelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

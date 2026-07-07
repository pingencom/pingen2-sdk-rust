use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeliveryProduct {
    Cheap,
    Fast,
    Registered,
    Bulk,
    Premium,
}

impl DeliveryProduct {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Cheap => "cheap",
            Self::Fast => "fast",
            Self::Registered => "registered",
            Self::Bulk => "bulk",
            Self::Premium => "premium",
        }
    }
}

impl fmt::Display for DeliveryProduct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatchIcon {
    Campaign,
    Megaphone,
    WaveHand,
    Flash,
    Rocket,
    Bell,
    PercentTag,
    PercentBadge,
    Present,
    Receipt,
    Document,
    Information,
    Calendar,
    Newspaper,
    Crown,
    Virus,
}

impl BatchIcon {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Campaign => "campaign",
            Self::Megaphone => "megaphone",
            Self::WaveHand => "wave-hand",
            Self::Flash => "flash",
            Self::Rocket => "rocket",
            Self::Bell => "bell",
            Self::PercentTag => "percent-tag",
            Self::PercentBadge => "percent-badge",
            Self::Present => "present",
            Self::Receipt => "receipt",
            Self::Document => "document",
            Self::Information => "information",
            Self::Calendar => "calendar",
            Self::Newspaper => "newspaper",
            Self::Crown => "crown",
            Self::Virus => "virus",
        }
    }
}

impl fmt::Display for BatchIcon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

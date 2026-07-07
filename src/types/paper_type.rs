use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaperType {
    Normal,
    Qr,
    SepaAt,
    SepaDe,
}

impl PaperType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Qr => "qr",
            Self::SepaAt => "sepa_at",
            Self::SepaDe => "sepa_de",
        }
    }
}

impl fmt::Display for PaperType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

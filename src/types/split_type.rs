use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitType {
    File,
    Page,
    Custom,
    QrInvoice,
}

impl SplitType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Page => "page",
            Self::Custom => "custom",
            Self::QrInvoice => "qr_invoice",
        }
    }
}

impl fmt::Display for SplitType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

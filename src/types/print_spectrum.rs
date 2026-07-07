use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrintSpectrum {
    Color,
    Grayscale,
}

impl PrintSpectrum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Color => "color",
            Self::Grayscale => "grayscale",
        }
    }
}

impl fmt::Display for PrintSpectrum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

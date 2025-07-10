use std::fmt::Display;

pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Reset,
}

impl Color {
    /// Returns the ANSI escape code for the color as a static string.
    #[inline]
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Red => "\x1b[31m",
            Self::Green => "\x1b[32m",
            Self::Yellow => "\x1b[33m",
            Self::Blue => "\x1b[34m",
            Self::Reset => "\x1b[0m",
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_color_as_str() {
        assert_eq!(Color::Red.as_str(), "\x1b[31m");
        assert_eq!(Color::Green.as_str(), "\x1b[32m");
        assert_eq!(Color::Yellow.as_str(), "\x1b[33m");
        assert_eq!(Color::Blue.as_str(), "\x1b[34m");
        assert_eq!(Color::Reset.as_str(), "\x1b[0m");
    }
}

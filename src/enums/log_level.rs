/// Represents the log level for a message.
#[derive(Debug)]
#[repr(u8)]
pub enum LogLevel {
    Error,
    Info,
}

impl LogLevel {
    /// Return a string representation of the log level
    #[inline]
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Error => "ERROR",
            Self::Info => "INFO",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::enums::log_level::LogLevel;

    #[tokio::test]
    async fn test_log_level_as_str() {
        assert_eq!(LogLevel::Error.as_str(), "ERROR");
        assert_eq!(LogLevel::Info.as_str(), "INFO");
    }

    #[tokio::test]
    async fn test_log_level_debug() {
        assert_eq!(format!("{:?}", LogLevel::Error), "Error");
        assert_eq!(format!("{:?}", LogLevel::Info), "Info");
    }
}

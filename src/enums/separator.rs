#[derive(Clone, Debug, PartialEq)]
pub enum SeparatorType {
    Comma,
    Semicolon,
    Tab,
    Pipe,
    Null,
    Invalid,
}

impl SeparatorType {
    pub(crate) fn as_char(&self) -> char {
        match self {
            SeparatorType::Comma => ',',
            SeparatorType::Semicolon => ';',
            SeparatorType::Tab => '\t',
            SeparatorType::Pipe => '|',
            SeparatorType::Null => '\0',
            SeparatorType::Invalid => ' ',
        }
    }
}

impl From<SeparatorType> for u8 {
    fn from(separator: SeparatorType) -> Self {
        match separator {
            SeparatorType::Comma => b',',
            SeparatorType::Semicolon => b';',
            SeparatorType::Tab => b'\t',
            SeparatorType::Pipe => b'|',
            SeparatorType::Null => b'\0',
            SeparatorType::Invalid => b' ',
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_separator_enum_as_char() {
        let comma: SeparatorType = SeparatorType::Comma;
        let semicolon: SeparatorType = SeparatorType::Semicolon;
        let tab: SeparatorType = SeparatorType::Tab;
        let pipe: SeparatorType = SeparatorType::Pipe;
        let null: SeparatorType = SeparatorType::Null;
        let invalid: SeparatorType = SeparatorType::Invalid;
        assert_eq!(comma.as_char(), ',');
        assert_eq!(semicolon.as_char(), ';');
        assert_eq!(tab.as_char(), '\t');
        assert_eq!(pipe.as_char(), '|');
        assert_eq!(null.as_char(), '\0');
        assert_eq!(invalid.as_char(), ' ');
    }

    #[tokio::test]
    async fn test_separator_enum_from_u8() {
        let comma: SeparatorType = SeparatorType::Comma;
        let semicolon: SeparatorType = SeparatorType::Semicolon;
        let tab: SeparatorType = SeparatorType::Tab;
        let pipe: SeparatorType = SeparatorType::Pipe;
        let null: SeparatorType = SeparatorType::Null;
        let invalid: SeparatorType = SeparatorType::Invalid;

        assert_eq!(u8::from(comma), b',');
        assert_eq!(u8::from(semicolon), b';');
        assert_eq!(u8::from(tab), b'\t');
        assert_eq!(u8::from(pipe), b'|');
        assert_eq!(u8::from(null), b'\0');
        assert_eq!(u8::from(invalid), b' ');
    }

    #[tokio::test]
    async fn test_separator_enum_clone() {
        let comma: SeparatorType = SeparatorType::Comma;
        let semicolon: SeparatorType = SeparatorType::Semicolon;
        let tab: SeparatorType = SeparatorType::Tab;
        let pipe: SeparatorType = SeparatorType::Pipe;
        let null: SeparatorType = SeparatorType::Null;
        let invalid: SeparatorType = SeparatorType::Invalid;
        assert_eq!(comma.clone(), SeparatorType::Comma);
        assert_eq!(semicolon.clone(), SeparatorType::Semicolon);
        assert_eq!(tab.clone(), SeparatorType::Tab);
        assert_eq!(pipe.clone(), SeparatorType::Pipe);
        assert_eq!(null.clone(), SeparatorType::Null);
        assert_eq!(invalid.clone(), SeparatorType::Invalid);
    }
}

use crate::utils::regex::safe_regex::{
    get_datetime_regex, get_email_regex, get_numeric_regex, get_phone_number_regex,
    get_simple_word_regex,
};
use regex::RegexSet;

pub mod safe_regex {
    use regex::Regex;

    /// Date and time pattern, supporting various formats
    #[inline]
    #[must_use]
    pub fn get_datetime_regex() -> Regex {
        Regex::new(
            r"(?i)\b(?:\d{4}[-/]\d{2}[-/]\d{2}|\d{2}[-/]\d{2}[-/]\d{4})\s?(?:\d{2}[:]\d{2}[:]\d{2})?\b",
        )
            .unwrap()
    }

    /// Numeric pattern, allowing for integers and decimals with optional signs
    #[inline]
    #[must_use]
    pub fn get_numeric_regex() -> Regex {
        Regex::new(r"^[-.]?\d+([.,]\d*)?\s*$").unwrap()
    }

    /// Email pattern, case-insensitive, allowing for common email formats
    #[inline]
    #[must_use]
    pub fn get_email_regex() -> Regex {
        Regex::new(r"(?i)\b[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Za-z]{2,}\b").unwrap()
    }

    /// Simple word pattern, allowing only letters (case-insensitive)
    #[inline]
    #[must_use]
    pub fn get_simple_word_regex() -> Regex {
        Regex::new(r"^[A-Za-z]+$").unwrap()
    }

    /// Phone number pattern, allowing for international formats
    #[inline]
    #[must_use]
    pub fn get_phone_number_regex() -> Regex {
        Regex::new(r"[+]?[0-9]{1,2}").unwrap()
    }

    #[cfg(test)]
    mod test {
        use crate::utils::regex::safe_regex::{
            get_datetime_regex, get_email_regex, get_numeric_regex, get_simple_word_regex,
        };
        use regex::Regex;

        #[tokio::test]
        async fn test_valid_dates() {
            const VALID_DATES: [&str; 8] = [
                "2024-02-03",
                "2024/02/03",
                "03-02-2024",
                "03/02/2024",
                "2024-02-03 12:34:56",
                "2024/02/03 23:59:59",
                "03-02-2024 00:00:00",
                "03/02/2024 07:45:30",
            ];
            let regex: Regex = get_datetime_regex();
            for date in &VALID_DATES {
                assert!(regex.is_match(date), "Erreur sur: {date}");
            }
        }

        #[tokio::test]
        async fn test_invalid_dates() {
            const INVALID_DATES: [&str; 3] = ["random text", "not a date", "is it a date?"];
            let regex: Regex = get_datetime_regex();

            for date in &INVALID_DATES {
                assert!(!regex.is_match(date), "Erreur sur: {date}");
            }
        }

        #[tokio::test]
        async fn test_valid_numbers() {
            const VALID_NUMBERS: [&str; 12] = [
                "123",
                "-123",
                "3.14",
                "0.99",
                "123,45",
                "-0.001",
                ".5",
                "-3.",
                "4579846541654",
                "0",
                "0.0",
                "-4645464664.6515",
            ];

            let regex: Regex = get_numeric_regex();

            for num in &VALID_NUMBERS {
                assert!(regex.is_match(num), "Erreur sur: {num}");
            }
        }

        #[tokio::test]
        async fn test_invalid_numbers() {
            let regex: Regex = get_numeric_regex();
            const INVALID_NUMBERS: [&str; 8] = [
                "abc", "123abc", "--3.14", "3..14", "3,14,15", "..5", "az4a4z6", "0.0.0",
            ];

            for num in &INVALID_NUMBERS {
                assert!(!regex.is_match(num), "Erreur sur: {num}");
            }
        }

        #[tokio::test]
        async fn test_valid_emails() {
            const VALID_EMAILS: [&str; 5] = [
                "test@example.com",
                "user.name+tag@domain.co.uk",
                "UPPERCASE@EMAIL.COM",
                "simple123@test.org",
                "a@b.io",
            ];

            let regex: Regex = get_email_regex();

            for email in &VALID_EMAILS {
                assert!(regex.is_match(email), "Erreur sur: {email}");
            }
        }

        #[tokio::test]
        async fn test_invalid_emails() {
            const INVALID_EMAILS: [&str; 6] = [
                "plainaddress",
                "@missinguser.com",
                "user@.com",
                "user@com",
                "user@domain,com",
                "user domain.com",
            ];

            let regex: Regex = get_email_regex();

            for email in &INVALID_EMAILS {
                assert!(!regex.is_match(email), "Erreur sur: {email}");
            }
        }

        #[tokio::test]
        async fn test_get_simple_word_regex() {
            const VALID_WORD: [&str; 6] = [
                "hello-world",
                "hello_world",
                "hello#world",
                "hello123world",
                "HéLLO",
                "NUMERI:",
            ];

            let regex: Regex = get_simple_word_regex();

            for email in &VALID_WORD {
                assert!(!regex.is_match(email), "Erreur sur: {email}");
            }
        }
    }
}

pub mod usafe_regex {
    /// Matches SQL keywords that could indicate an injection attempt
    #[inline]
    #[must_use]
    pub const fn sql_keyword_regex() -> &'static str {
        r"(?i)\b(SELECT|INSERT|UPDATE|DELETE|DROP|TRUNCATE|EXEC|UNION|ALTER|CREATE|REPLACE|MERGE|CALL|DECLARE|CAST)\b"
    }

    /// Matches any suspicious or illegal character
    #[inline]
    #[must_use]
    pub const fn illegal_char_regex() -> &'static str {
        r"[^\w\s]"
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use regex::Regex;

        #[tokio::test]
        async fn test_sql_keyword_regex() {
            const SQL_KEYWORDS: [&str; 5] = ["SELECT", "INSERT", "UPDATE", "DELETE", "DROP"];
            let regex = sql_keyword_regex();
            let regex: Regex = Regex::new(regex).unwrap();

            for keyword in &SQL_KEYWORDS {
                assert!(regex.is_match(keyword), "Error on : {keyword}");
            }
        }

        #[tokio::test]
        async fn test_illegal_char_regex() {
            let regex = illegal_char_regex();
            let regex: Regex = Regex::new(regex).unwrap();
            const ILLEGAL_CHARS: [&str; 5] = [
                "@#$%^&*()",
                "<script>",
                "alert('XSS')",
                "' OR '1'='1",
                "\" OR \"a\"=\"a",
            ];

            for char in &ILLEGAL_CHARS {
                assert!(regex.is_match(char), "Error on : {char}");
            }
        }
    }
}

/// Return a `RegexSet` for unsafe values
#[inline]
#[must_use]
pub fn get_safe_regex_set() -> RegexSet {
    RegexSet::new([
        get_numeric_regex().as_str(),
        get_datetime_regex().as_str(),
        get_email_regex().as_str(),
        get_simple_word_regex().as_str(),
        get_phone_number_regex().as_str(),
    ])
    .unwrap()
}

/// Return a `RegexSet` for unsafe values
#[inline]
#[must_use]
pub fn get_unsafe_value_regex_set() -> RegexSet {
    RegexSet::new([
        usafe_regex::sql_keyword_regex(),
        usafe_regex::illegal_char_regex(),
    ])
    .unwrap()
}

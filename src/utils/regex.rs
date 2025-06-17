use crate::utils::regex::safe_regex::*;
use regex::RegexSet;

pub mod safe_regex {
    use regex::Regex;

    /// Pattern pour les dates au format YYYY-MM-DD ou DD-MM-YYYY
    pub fn get_datetime_regex() -> Regex {
        Regex::new(
            r"(?i)\b(?:\d{4}[-/]\d{2}[-/]\d{2}|\d{2}[-/]\d{2}[-/]\d{4})\s?(?:\d{2}[:]\d{2}[:]\d{2})?\b",
        )
            .unwrap()
    }

    /// Pattern pour les nombres, y compris les nombres négatifs et décimaux
    pub fn get_numeric_regex() -> Regex {
        Regex::new(r"^[-.]?\d+([.,]\d*)?\s*$").unwrap()
    }

    /// Pattern pour les adresses e-mail
    pub fn get_email_regex() -> Regex {
        Regex::new(r"(?i)\b[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Za-z]{2,}\b").unwrap()
    }

    /// Pattern pour les mots simples (sans espaces ni caractères spéciaux)
    pub fn get_simple_word_regex() -> Regex {
        Regex::new(r"^[A-Za-z]+$").unwrap()
    }

    /// Pattern pour les numéros de téléphone (ex: +33, 01, 06, etc.)
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
            let regex: Regex = get_datetime_regex();
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

            for date in VALID_DATES.iter() {
                assert!(regex.is_match(date), "Erreur sur: {date}");
            }
        }

        #[tokio::test]
        async fn test_invalid_dates() {
            let regex: Regex = get_datetime_regex();
            const INVALID_DATES: [&str; 3] = ["random text", "not a date", "is it a date?"];

            for date in INVALID_DATES.iter() {
                assert!(!regex.is_match(date), "Erreur sur: {date}");
            }
        }

        #[tokio::test]
        async fn test_valid_numbers() {
            let regex: Regex = get_numeric_regex();
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

            for num in VALID_NUMBERS.iter() {
                assert!(regex.is_match(num), "Erreur sur: {num}");
            }
        }

        #[tokio::test]
        async fn test_invalid_numbers() {
            let regex: Regex = get_numeric_regex();
            const INVALID_NUMBERS: [&str; 8] = [
                "abc", "123abc", "--3.14", "3..14", "3,14,15", "..5", "az4a4z6", "0.0.0",
            ];

            for num in INVALID_NUMBERS.iter() {
                assert!(!regex.is_match(num), "Erreur sur: {num}");
            }
        }

        #[tokio::test]
        async fn test_valid_emails() {
            let regex: Regex = get_email_regex();
            const VALID_EMAILS: [&str; 5] = [
                "test@example.com",
                "user.name+tag@domain.co.uk",
                "UPPERCASE@EMAIL.COM",
                "simple123@test.org",
                "a@b.io",
            ];

            for email in VALID_EMAILS.iter() {
                assert!(regex.is_match(email), "Erreur sur: {email}");
            }
        }

        #[tokio::test]
        async fn test_invalid_emails() {
            let regex: Regex = get_email_regex();
            const INVALID_EMAILS: [&str; 6] = [
                "plainaddress",
                "@missinguser.com",
                "user@.com",
                "user@com",
                "user@domain,com",
                "user domain.com",
            ];

            for email in INVALID_EMAILS.iter() {
                assert!(!regex.is_match(email), "Erreur sur: {email}");
            }
        }

        #[tokio::test]
        async fn test_get_simple_word_regex() {
            let regex: Regex = get_simple_word_regex();
            const VALID_WORD: [&str; 6] = [
                "hello-world",
                "hello_world",
                "hello#world",
                "hello123world",
                "HéLLO",
                "NUMERI:",
            ];

            for email in VALID_WORD.iter() {
                assert!(!regex.is_match(email), "Erreur sur: {email}");
            }
        }
    }
}

pub mod usafe_regex {
    /// Matches common SQL keywords that could be unsafe
    pub fn sql_keyword_regex() -> String {
        r"(?i)\b(SELECT|INSERT|UPDATE|DELETE|DROP|TRUNCATE|EXEC|UNION|ALTER|CREATE|REPLACE|MERGE|CALL|DECLARE|CAST)\b".into()
    }

    /// Matches any suspicious or illegal character
    pub fn illegal_char_regex() -> String {
        r"[^\w\s]".into()
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use regex::Regex;

        #[tokio::test]
        async fn test_sql_keyword_regex() {
            let regex: String = sql_keyword_regex();
            let regex: Regex = Regex::new(&regex).unwrap();
            const SQL_KEYWORDS: [&str; 5] = ["SELECT", "INSERT", "UPDATE", "DELETE", "DROP"];

            for keyword in SQL_KEYWORDS.iter() {
                assert!(regex.is_match(keyword), "Erreur sur: {keyword}");
            }
        }

        #[tokio::test]
        async fn test_illegal_char_regex() {
            let regex: String = illegal_char_regex();
            let regex: Regex = Regex::new(&regex).unwrap();
            const ILLEGAL_CHARS: [&str; 5] = [
                "@#$%^&*()",
                "<script>",
                "alert('XSS')",
                "' OR '1'='1",
                "\" OR \"a\"=\"a",
            ];

            for char in ILLEGAL_CHARS.iter() {
                assert!(regex.is_match(char), "Erreur sur: {char}");
            }
        }
    }
}

/// Fonction pour obtenir un ensemble de regex
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

/// Fonction pour obtenir une regex pour les valeurs dangereuses
pub fn get_unsafe_value_regex_set() -> RegexSet {
    RegexSet::new([
        usafe_regex::sql_keyword_regex().as_str(),
        usafe_regex::illegal_char_regex().as_str(),
    ])
    .unwrap()
}

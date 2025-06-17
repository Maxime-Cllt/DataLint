use crate::structs::color::{BLUE, GREEN, RED, RESET, YELLOW};

#[tokio::test]
async fn test_red_color_code() {
    assert_eq!(RED, "\x1b[31m");
    assert_eq!(GREEN, "\x1b[32m");
    assert_eq!(YELLOW, "\x1b[33m");
    assert_eq!(BLUE, "\x1b[34m");
    assert_eq!(RESET, "\x1b[0m");
}

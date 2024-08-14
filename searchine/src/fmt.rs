const RED_CODE: &str = "\x1b[31m";
const GREEN_CODE: &str = "\x1b[32m";
const RESET_CODE: &str = "\x1b[0m";

/// Print text in red color.
pub fn fmt_red(text: &str) -> String {
    format!("{}{}{}", RED_CODE, text, RESET_CODE)
}

/// Print text in green color.
pub fn fmt_green(text: &str) -> String {
    format!("{}{}{}", GREEN_CODE, text, RESET_CODE)
}

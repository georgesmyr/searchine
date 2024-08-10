pub mod init;
pub mod index_corpus;
pub mod list_corpus;
pub mod create_vocabulary;
pub mod index;
pub mod search;


/// Creates a hyperlink, by specifying the path it points to and
/// the text to be displayed. The hyperlink is formatted in blue
/// bold and underlying.
fn format_hyperlink(path: &std::path::PathBuf, text: &str) -> String {
    let bold_blue_underline = "\x1b[1m\x1b[34m\x1b[4m";
    let reset = "\x1b[0m";
    format!(
        "\x1B]8;;file://{}\x1B\\{}{}{}\x1B]8;;\x1B\\",
        path.to_str().unwrap(),
        bold_blue_underline,
        text,
        reset
    )
}


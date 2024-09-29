use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use xml::reader::XmlEvent;
use xml::EventReader;

use crate::core::dtype::DocumentType;

/// Reads the contents of a file and returns it as a string.
///
/// This function determines the type of the file based on its path and reads its
/// contents accordingly. If the file type is unsupported, it returns an error.
///
/// # Arguments
///
/// * `path` - A path to the file to be read.
///
/// # Returns
///
/// * `std::io::Result<String>` - The contents of the file as a string, or an error
///   if the file type is unsupported or if there is an I/O error.
///
/// # Errors
///
/// This function will return an error if the file type is unsupported or if there
/// is an I/O error.
pub fn read_to_string(path: impl AsRef<Path>) -> std::io::Result<String> {
    match DocumentType::from_path(&path) {
        Some(DocumentType::Xml) => read_xml_file(path),
        Some(DocumentType::Text) => read_text_file(path),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Unsupported file type",
        )),
    }
}

/// Reads an XML file and returns its contents (characters) as a string.
fn read_xml_file(path: impl AsRef<Path>) -> std::io::Result<String> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let er = EventReader::new(reader);
    let mut contents = String::new();
    for event in er.into_iter().flatten() {
        if let XmlEvent::Characters(string) = event {
            contents.push_str(&string);
            contents.push(' ');
        }
    }
    Ok(contents)
}

/// Reads a text file and returns its contents as a string.
fn read_text_file(path: impl AsRef<Path>) -> std::io::Result<String> {
    let file = File::open(path)?;
    let size = file.metadata().map(|m| m.len() as usize).ok();
    let mut string = String::new();
    string.try_reserve_exact(size.unwrap_or(0))?;
    let mut reader = BufReader::new(file);
    reader.read_to_string(&mut string).ok();
    Ok(string)
}

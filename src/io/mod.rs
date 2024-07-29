use std::path::Path;

use xml::reader::{EventReader, XmlEvent};

use crate::fs::FileType;

pub struct FileReader;
impl FileReader {
    pub fn read(path: impl AsRef<Path>) -> std::io::Result<String> {
        let file_type = FileType::from_path(&path);
        match file_type {
            Some(FileType::Xml) => read_xml_file(path),
            Some(FileType::Text) => read_text_file(path),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Unsupported file type",
            )),
        }
    }
}

/// Reads an XML file and returns its contents (characters) as a string.
pub fn read_xml_file(path: impl AsRef<Path>) -> std::io::Result<String> {
    let file = std::fs::File::open(path)?;
    let mut er = EventReader::new(file);
    let mut contents = String::new();
    for event in er {
        if let Ok(XmlEvent::Characters(string)) = event {
            contents.push_str(" ");
            contents.push_str(&string);
        }
    }
    Ok(contents)
}


/// Reads a text file and returns its contents as a string.
pub fn read_text_file(path: impl AsRef<Path>) -> std::io::Result<String> {
    let contents = std::fs::read_to_string(path)?;
    Ok(contents)
}
use std::ffi::OsStr;
use std::path::Path;

use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, PartialEq)]
pub enum DocumentType {
    Xml,
    Text,
    Html,
    Pdf,
}

impl DocumentType {
    /// Returns the file type based on the extension of the file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::ffi::OsStr;
    /// use fs::DocumentType;
    ///
    /// let ext = OsStr::new("txt");
    /// let file_type = DocumentType::from_extension(ext);
    /// assert_eq!(file_type, Some(FileType::Text));
    /// ```
    pub fn from_extension(ext: &OsStr) -> Option<Self> {
        match ext.to_str().unwrap() {
            "xhtml" => Some(Self::Xml),
            "txt" | "md" => Some(Self::Text),
            "html" => Some(Self::Html),
            "pdf" => Some(Self::Pdf),
            _ => None,
        }
    }

    /// Returns the file type based on the path of the file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use fs::DocumentType;
    ///
    /// let path = Path::new("file.txt");
    /// let file_type = DocumentType::from_path(path);
    /// assert_eq!(file_type, Some(FileType::Text));
    /// ```
    pub fn from_path(path: impl AsRef<Path>) -> Option<Self> {
        let extension = path.as_ref().extension();
        match extension {
            Some(ext) => Self::from_extension(ext),
            None => None,
        }
    }
}
pub struct Document;
impl Document {
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
}

/// Reads an XML file and returns its contents (characters) as a string.
fn read_xml_file(path: impl AsRef<Path>) -> std::io::Result<String> {
    let file = std::fs::File::open(path)?;
    let er = EventReader::new(file);
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
fn read_text_file(path: impl AsRef<Path>) -> std::io::Result<String> {
    let contents = std::fs::read_to_string(path)?;
    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_type_from_extension() {
        assert_eq!(
            DocumentType::from_extension(OsStr::new("xhtml")),
            Some(DocumentType::Xml)
        );
        assert_eq!(
            DocumentType::from_extension(OsStr::new("txt")),
            Some(DocumentType::Text)
        );
        assert_eq!(
            DocumentType::from_extension(OsStr::new("md")),
            Some(DocumentType::Text)
        );
        assert_eq!(
            DocumentType::from_extension(OsStr::new("html")),
            Some(DocumentType::Html)
        );
        assert_eq!(
            DocumentType::from_extension(OsStr::new("pdf")),
            Some(DocumentType::Pdf)
        );
        assert_eq!(DocumentType::from_extension(OsStr::new("")), None);
        assert_eq!(DocumentType::from_extension(OsStr::new("jpg")), None);
    }
}

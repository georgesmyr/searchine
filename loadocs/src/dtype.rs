use std::ffi::OsStr;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum DocumentType {
    Xml,
    Text,
    Pdf,
}

impl DocumentType {
    /// Returns the file type based on the extension of the file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::ffi::OsStr;
    /// use loadocs::DocumentType;
    ///
    /// let ext = OsStr::new("txt");
    /// let file_type = DocumentType::from_extension(ext);
    /// assert_eq!(file_type, Some(DocumentType::Text));
    /// ```
    pub fn from_extension(ext: &OsStr) -> Option<Self> {
        match ext.to_str().unwrap() {
            "xhtml" | "html" | "xml" => Some(Self::Xml),
            "txt" | "md" => Some(Self::Text),
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
    /// use loadocs::DocumentType;
    ///
    /// let path = Path::new("file.txt");
    /// let file_type = DocumentType::from_path(path);
    /// assert_eq!(file_type, Some(DocumentType::Text));
    /// ```
    pub fn from_path(path: impl AsRef<Path>) -> Option<Self> {
        let extension = path.as_ref().extension();
        match extension {
            Some(ext) => Self::from_extension(ext),
            None => None,
        }
    }
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
            DocumentType::from_extension(OsStr::new("pdf")),
            Some(DocumentType::Pdf)
        );
        assert_eq!(DocumentType::from_extension(OsStr::new("")), None);
        assert_eq!(DocumentType::from_extension(OsStr::new("jpg")), None);
    }
}

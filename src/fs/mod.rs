use std::ffi::OsStr;
use std::path::Path;

/// The type of file to read.
#[derive(Debug, PartialEq)]
pub enum FileType {
    Xml,
    Text,
    Html,
    Pdf,
}

impl FileType {
    /// Returns the file type based on the extension of the file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::ffi::OsStr;
    /// use fs::FileType;
    ///
    /// let ext = OsStr::new("txt");
    /// let file_type = FileType::from_extension(ext);
    /// assert_eq!(file_type, Some(FileType::Text));
    /// ```
    pub fn from_extension(ext: &OsStr) -> Option<FileType> {
        match ext.to_str().unwrap() {
            "xhtml" => Some(FileType::Xml),
            "txt" | "md" => Some(FileType::Text),
            "html" => Some(FileType::Html),
            "pdf" => Some(FileType::Pdf),
            _ => None,
        }
    }

    /// Returns the file type based on the path of the file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use fs::FileType;
    ///
    /// let path = Path::new("file.txt");
    /// let file_type = FileType::from_path(path);
    /// assert_eq!(file_type, Some(FileType::Text));
    /// ```
    pub fn from_path(path: impl AsRef<Path>) -> Option<FileType> {
        let extension = path.as_ref().extension();
        match extension {
            Some(ext) => FileType::from_extension(ext),
            None => None,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_type_from_extension() {
        assert_eq!(FileType::from_extension(OsStr::new("xhtml")), Some(FileType::Xml));
        assert_eq!(FileType::from_extension(OsStr::new("txt")), Some(FileType::Text));
        assert_eq!(FileType::from_extension(OsStr::new("md")), Some(FileType::Text));
        assert_eq!(FileType::from_extension(OsStr::new("html")), Some(FileType::Html));
        assert_eq!(FileType::from_extension(OsStr::new("pdf")), Some(FileType::Pdf));
        assert_eq!(FileType::from_extension(OsStr::new("")), None);
        assert_eq!(FileType::from_extension(OsStr::new("jpg")), None);
    }
}

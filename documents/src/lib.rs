pub use crate::core::document::Document;
pub use crate::core::dtype::DocumentType;
pub use crate::core::metadata::{DocumentMetadata, DocumentSource};
pub use crate::read::read_to_string;

pub type DocumentId = u32;

pub mod core;
pub mod read;
pub mod split;

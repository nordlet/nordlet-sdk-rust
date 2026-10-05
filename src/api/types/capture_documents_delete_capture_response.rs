pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DocumentsDeleteCaptureResponse {
    #[serde(default)]
    pub deleted: bool,
}

impl DocumentsDeleteCaptureResponse {
    pub fn builder() -> DocumentsDeleteCaptureResponseBuilder {
        <DocumentsDeleteCaptureResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentsDeleteCaptureResponseBuilder {
    deleted: Option<bool>,
}

impl DocumentsDeleteCaptureResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DocumentsDeleteCaptureResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DocumentsDeleteCaptureResponseBuilder::deleted)
    pub fn build(self) -> Result<DocumentsDeleteCaptureResponse, BuildError> {
        Ok(DocumentsDeleteCaptureResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}

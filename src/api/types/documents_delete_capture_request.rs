pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DocumentsDeleteCaptureRequest {
    #[serde(default)]
    pub id: String,
}

impl DocumentsDeleteCaptureRequest {
    pub fn builder() -> DocumentsDeleteCaptureRequestBuilder {
        <DocumentsDeleteCaptureRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentsDeleteCaptureRequestBuilder {
    id: Option<String>,
}

impl DocumentsDeleteCaptureRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DocumentsDeleteCaptureRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DocumentsDeleteCaptureRequestBuilder::id)
    pub fn build(self) -> Result<DocumentsDeleteCaptureRequest, BuildError> {
        Ok(DocumentsDeleteCaptureRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

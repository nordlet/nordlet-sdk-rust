pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DocumentsExtractCaptureRequest {
    #[serde(default)]
    pub id: String,
}

impl DocumentsExtractCaptureRequest {
    pub fn builder() -> DocumentsExtractCaptureRequestBuilder {
        <DocumentsExtractCaptureRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentsExtractCaptureRequestBuilder {
    id: Option<String>,
}

impl DocumentsExtractCaptureRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DocumentsExtractCaptureRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DocumentsExtractCaptureRequestBuilder::id)
    pub fn build(self) -> Result<DocumentsExtractCaptureRequest, BuildError> {
        Ok(DocumentsExtractCaptureRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DocumentsGetCaptureRequest {
    #[serde(default)]
    pub id: String,
}

impl DocumentsGetCaptureRequest {
    pub fn builder() -> DocumentsGetCaptureRequestBuilder {
        <DocumentsGetCaptureRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentsGetCaptureRequestBuilder {
    id: Option<String>,
}

impl DocumentsGetCaptureRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DocumentsGetCaptureRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DocumentsGetCaptureRequestBuilder::id)
    pub fn build(self) -> Result<DocumentsGetCaptureRequest, BuildError> {
        Ok(DocumentsGetCaptureRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteDocumentSeriesRequest {
    #[serde(default)]
    pub id: String,
}

impl DeleteDocumentSeriesRequest {
    pub fn builder() -> DeleteDocumentSeriesRequestBuilder {
        <DeleteDocumentSeriesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteDocumentSeriesRequestBuilder {
    id: Option<String>,
}

impl DeleteDocumentSeriesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteDocumentSeriesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DeleteDocumentSeriesRequestBuilder::id)
    pub fn build(self) -> Result<DeleteDocumentSeriesRequest, BuildError> {
        Ok(DeleteDocumentSeriesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

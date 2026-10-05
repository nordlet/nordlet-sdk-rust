pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetDocumentSeriesRequest {
    #[serde(default)]
    pub id: String,
}

impl GetDocumentSeriesRequest {
    pub fn builder() -> GetDocumentSeriesRequestBuilder {
        <GetDocumentSeriesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetDocumentSeriesRequestBuilder {
    id: Option<String>,
}

impl GetDocumentSeriesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetDocumentSeriesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetDocumentSeriesRequestBuilder::id)
    pub fn build(self) -> Result<GetDocumentSeriesRequest, BuildError> {
        Ok(GetDocumentSeriesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

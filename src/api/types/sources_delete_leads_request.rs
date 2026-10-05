pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SourcesDeleteLeadsRequest {
    #[serde(default)]
    pub id: String,
}

impl SourcesDeleteLeadsRequest {
    pub fn builder() -> SourcesDeleteLeadsRequestBuilder {
        <SourcesDeleteLeadsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SourcesDeleteLeadsRequestBuilder {
    id: Option<String>,
}

impl SourcesDeleteLeadsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SourcesDeleteLeadsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SourcesDeleteLeadsRequestBuilder::id)
    pub fn build(self) -> Result<SourcesDeleteLeadsRequest, BuildError> {
        Ok(SourcesDeleteLeadsRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

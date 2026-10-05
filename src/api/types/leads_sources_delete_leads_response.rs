pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SourcesDeleteLeadsResponse {
    #[serde(default)]
    pub id: String,
}

impl SourcesDeleteLeadsResponse {
    pub fn builder() -> SourcesDeleteLeadsResponseBuilder {
        <SourcesDeleteLeadsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SourcesDeleteLeadsResponseBuilder {
    id: Option<String>,
}

impl SourcesDeleteLeadsResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SourcesDeleteLeadsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SourcesDeleteLeadsResponseBuilder::id)
    pub fn build(self) -> Result<SourcesDeleteLeadsResponse, BuildError> {
        Ok(SourcesDeleteLeadsResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

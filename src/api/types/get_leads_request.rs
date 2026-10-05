pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetLeadsRequest {
    #[serde(default)]
    pub id: String,
}

impl GetLeadsRequest {
    pub fn builder() -> GetLeadsRequestBuilder {
        <GetLeadsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetLeadsRequestBuilder {
    id: Option<String>,
}

impl GetLeadsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetLeadsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetLeadsRequestBuilder::id)
    pub fn build(self) -> Result<GetLeadsRequest, BuildError> {
        Ok(GetLeadsRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

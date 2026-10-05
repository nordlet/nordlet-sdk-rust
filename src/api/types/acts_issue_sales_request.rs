pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ActsIssueSalesRequest {
    #[serde(default)]
    pub id: String,
}

impl ActsIssueSalesRequest {
    pub fn builder() -> ActsIssueSalesRequestBuilder {
        <ActsIssueSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ActsIssueSalesRequestBuilder {
    id: Option<String>,
}

impl ActsIssueSalesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ActsIssueSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ActsIssueSalesRequestBuilder::id)
    pub fn build(self) -> Result<ActsIssueSalesRequest, BuildError> {
        Ok(ActsIssueSalesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

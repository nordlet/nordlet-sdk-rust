pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ActsGetSalesRequest {
    #[serde(default)]
    pub id: String,
}

impl ActsGetSalesRequest {
    pub fn builder() -> ActsGetSalesRequestBuilder {
        <ActsGetSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ActsGetSalesRequestBuilder {
    id: Option<String>,
}

impl ActsGetSalesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ActsGetSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ActsGetSalesRequestBuilder::id)
    pub fn build(self) -> Result<ActsGetSalesRequest, BuildError> {
        Ok(ActsGetSalesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

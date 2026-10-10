pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetPlatformSellersRequest {
    #[serde(default)]
    pub id: String,
}

impl GetPlatformSellersRequest {
    pub fn builder() -> GetPlatformSellersRequestBuilder {
        <GetPlatformSellersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPlatformSellersRequestBuilder {
    id: Option<String>,
}

impl GetPlatformSellersRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetPlatformSellersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetPlatformSellersRequestBuilder::id)
    pub fn build(self) -> Result<GetPlatformSellersRequest, BuildError> {
        Ok(GetPlatformSellersRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

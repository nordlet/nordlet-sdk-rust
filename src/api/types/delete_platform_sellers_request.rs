pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeletePlatformSellersRequest {
    #[serde(default)]
    pub id: String,
}

impl DeletePlatformSellersRequest {
    pub fn builder() -> DeletePlatformSellersRequestBuilder {
        <DeletePlatformSellersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeletePlatformSellersRequestBuilder {
    id: Option<String>,
}

impl DeletePlatformSellersRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeletePlatformSellersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DeletePlatformSellersRequestBuilder::id)
    pub fn build(self) -> Result<DeletePlatformSellersRequest, BuildError> {
        Ok(DeletePlatformSellersRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

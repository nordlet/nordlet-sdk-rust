pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeletePlatformSellersResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub deleted: bool,
}

impl DeletePlatformSellersResponse {
    pub fn builder() -> DeletePlatformSellersResponseBuilder {
        <DeletePlatformSellersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeletePlatformSellersResponseBuilder {
    id: Option<String>,
    deleted: Option<bool>,
}

impl DeletePlatformSellersResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeletePlatformSellersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DeletePlatformSellersResponseBuilder::id)
    /// - [`deleted`](DeletePlatformSellersResponseBuilder::deleted)
    pub fn build(self) -> Result<DeletePlatformSellersResponse, BuildError> {
        Ok(DeletePlatformSellersResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}

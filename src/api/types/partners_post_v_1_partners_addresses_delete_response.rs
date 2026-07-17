pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersAddressesDeleteResponse {
    #[serde(default)]
    pub deleted: bool,
}

impl PostV1PartnersAddressesDeleteResponse {
    pub fn builder() -> PostV1PartnersAddressesDeleteResponseBuilder {
        <PostV1PartnersAddressesDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersAddressesDeleteResponseBuilder {
    deleted: Option<bool>,
}

impl PostV1PartnersAddressesDeleteResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersAddressesDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](PostV1PartnersAddressesDeleteResponseBuilder::deleted)
    pub fn build(self) -> Result<PostV1PartnersAddressesDeleteResponse, BuildError> {
        Ok(PostV1PartnersAddressesDeleteResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}

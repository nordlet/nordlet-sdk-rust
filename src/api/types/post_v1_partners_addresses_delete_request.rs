pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersAddressesDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1PartnersAddressesDeleteRequest {
    pub fn builder() -> PostV1PartnersAddressesDeleteRequestBuilder {
        <PostV1PartnersAddressesDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersAddressesDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1PartnersAddressesDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersAddressesDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersAddressesDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1PartnersAddressesDeleteRequest, BuildError> {
        Ok(PostV1PartnersAddressesDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

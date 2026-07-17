pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersContactsDeleteResponse {
    #[serde(default)]
    pub deleted: bool,
}

impl PostV1PartnersContactsDeleteResponse {
    pub fn builder() -> PostV1PartnersContactsDeleteResponseBuilder {
        <PostV1PartnersContactsDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersContactsDeleteResponseBuilder {
    deleted: Option<bool>,
}

impl PostV1PartnersContactsDeleteResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersContactsDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](PostV1PartnersContactsDeleteResponseBuilder::deleted)
    pub fn build(self) -> Result<PostV1PartnersContactsDeleteResponse, BuildError> {
        Ok(PostV1PartnersContactsDeleteResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}

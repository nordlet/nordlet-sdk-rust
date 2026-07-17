pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersContactsDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1PartnersContactsDeleteRequest {
    pub fn builder() -> PostV1PartnersContactsDeleteRequestBuilder {
        <PostV1PartnersContactsDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersContactsDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1PartnersContactsDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersContactsDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersContactsDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1PartnersContactsDeleteRequest, BuildError> {
        Ok(PostV1PartnersContactsDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

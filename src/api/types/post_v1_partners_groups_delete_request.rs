pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersGroupsDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1PartnersGroupsDeleteRequest {
    pub fn builder() -> PostV1PartnersGroupsDeleteRequestBuilder {
        <PostV1PartnersGroupsDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersGroupsDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1PartnersGroupsDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersGroupsDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersGroupsDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1PartnersGroupsDeleteRequest, BuildError> {
        Ok(PostV1PartnersGroupsDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

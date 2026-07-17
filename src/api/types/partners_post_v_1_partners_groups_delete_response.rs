pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersGroupsDeleteResponse {
    #[serde(default)]
    pub id: String,
}

impl PostV1PartnersGroupsDeleteResponse {
    pub fn builder() -> PostV1PartnersGroupsDeleteResponseBuilder {
        <PostV1PartnersGroupsDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersGroupsDeleteResponseBuilder {
    id: Option<String>,
}

impl PostV1PartnersGroupsDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersGroupsDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersGroupsDeleteResponseBuilder::id)
    pub fn build(self) -> Result<PostV1PartnersGroupsDeleteResponse, BuildError> {
        Ok(PostV1PartnersGroupsDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

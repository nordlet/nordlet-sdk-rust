pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountMembersRemoveRequest {
    #[serde(rename = "userId")]
    #[serde(default)]
    pub user_id: String,
}

impl PostV1AccountMembersRemoveRequest {
    pub fn builder() -> PostV1AccountMembersRemoveRequestBuilder {
        <PostV1AccountMembersRemoveRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountMembersRemoveRequestBuilder {
    user_id: Option<String>,
}

impl PostV1AccountMembersRemoveRequestBuilder {
    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountMembersRemoveRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_id`](PostV1AccountMembersRemoveRequestBuilder::user_id)
    pub fn build(self) -> Result<PostV1AccountMembersRemoveRequest, BuildError> {
        Ok(PostV1AccountMembersRemoveRequest {
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
        })
    }
}

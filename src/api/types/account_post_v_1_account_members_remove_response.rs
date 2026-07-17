pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountMembersRemoveResponse {
    #[serde(default)]
    pub removed: bool,
}

impl PostV1AccountMembersRemoveResponse {
    pub fn builder() -> PostV1AccountMembersRemoveResponseBuilder {
        <PostV1AccountMembersRemoveResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountMembersRemoveResponseBuilder {
    removed: Option<bool>,
}

impl PostV1AccountMembersRemoveResponseBuilder {
    pub fn removed(mut self, value: bool) -> Self {
        self.removed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountMembersRemoveResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`removed`](PostV1AccountMembersRemoveResponseBuilder::removed)
    pub fn build(self) -> Result<PostV1AccountMembersRemoveResponse, BuildError> {
        Ok(PostV1AccountMembersRemoveResponse {
            removed: self
                .removed
                .ok_or_else(|| BuildError::missing_field("removed"))?,
        })
    }
}

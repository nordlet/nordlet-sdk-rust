pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountDeleteResponse {
    #[serde(default)]
    pub deleted: bool,
}

impl PostV1AccountDeleteResponse {
    pub fn builder() -> PostV1AccountDeleteResponseBuilder {
        <PostV1AccountDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountDeleteResponseBuilder {
    deleted: Option<bool>,
}

impl PostV1AccountDeleteResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](PostV1AccountDeleteResponseBuilder::deleted)
    pub fn build(self) -> Result<PostV1AccountDeleteResponse, BuildError> {
        Ok(PostV1AccountDeleteResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}

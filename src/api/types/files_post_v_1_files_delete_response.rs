pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1FilesDeleteResponse {
    #[serde(default)]
    pub deleted: bool,
}

impl PostV1FilesDeleteResponse {
    pub fn builder() -> PostV1FilesDeleteResponseBuilder {
        <PostV1FilesDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1FilesDeleteResponseBuilder {
    deleted: Option<bool>,
}

impl PostV1FilesDeleteResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1FilesDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](PostV1FilesDeleteResponseBuilder::deleted)
    pub fn build(self) -> Result<PostV1FilesDeleteResponse, BuildError> {
        Ok(PostV1FilesDeleteResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}

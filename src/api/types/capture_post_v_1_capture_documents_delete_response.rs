pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CaptureDocumentsDeleteResponse {
    #[serde(default)]
    pub deleted: bool,
}

impl PostV1CaptureDocumentsDeleteResponse {
    pub fn builder() -> PostV1CaptureDocumentsDeleteResponseBuilder {
        <PostV1CaptureDocumentsDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CaptureDocumentsDeleteResponseBuilder {
    deleted: Option<bool>,
}

impl PostV1CaptureDocumentsDeleteResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CaptureDocumentsDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](PostV1CaptureDocumentsDeleteResponseBuilder::deleted)
    pub fn build(self) -> Result<PostV1CaptureDocumentsDeleteResponse, BuildError> {
        Ok(PostV1CaptureDocumentsDeleteResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}

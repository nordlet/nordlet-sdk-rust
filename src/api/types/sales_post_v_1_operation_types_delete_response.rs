pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1OperationTypesDeleteResponse {
    #[serde(default)]
    pub deleted: bool,
}

impl PostV1OperationTypesDeleteResponse {
    pub fn builder() -> PostV1OperationTypesDeleteResponseBuilder {
        <PostV1OperationTypesDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1OperationTypesDeleteResponseBuilder {
    deleted: Option<bool>,
}

impl PostV1OperationTypesDeleteResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1OperationTypesDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](PostV1OperationTypesDeleteResponseBuilder::deleted)
    pub fn build(self) -> Result<PostV1OperationTypesDeleteResponse, BuildError> {
        Ok(PostV1OperationTypesDeleteResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}

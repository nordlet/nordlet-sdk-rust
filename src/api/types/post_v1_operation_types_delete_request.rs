pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1OperationTypesDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1OperationTypesDeleteRequest {
    pub fn builder() -> PostV1OperationTypesDeleteRequestBuilder {
        <PostV1OperationTypesDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1OperationTypesDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1OperationTypesDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1OperationTypesDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1OperationTypesDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1OperationTypesDeleteRequest, BuildError> {
        Ok(PostV1OperationTypesDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1OperationTypesGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1OperationTypesGetRequest {
    pub fn builder() -> PostV1OperationTypesGetRequestBuilder {
        <PostV1OperationTypesGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1OperationTypesGetRequestBuilder {
    id: Option<String>,
}

impl PostV1OperationTypesGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1OperationTypesGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1OperationTypesGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1OperationTypesGetRequest, BuildError> {
        Ok(PostV1OperationTypesGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

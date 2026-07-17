pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesActsCancelRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1SalesActsCancelRequest {
    pub fn builder() -> PostV1SalesActsCancelRequestBuilder {
        <PostV1SalesActsCancelRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesActsCancelRequestBuilder {
    id: Option<String>,
}

impl PostV1SalesActsCancelRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesActsCancelRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1SalesActsCancelRequestBuilder::id)
    pub fn build(self) -> Result<PostV1SalesActsCancelRequest, BuildError> {
        Ok(PostV1SalesActsCancelRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

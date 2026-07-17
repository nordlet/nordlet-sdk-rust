pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesActsGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1SalesActsGetRequest {
    pub fn builder() -> PostV1SalesActsGetRequestBuilder {
        <PostV1SalesActsGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesActsGetRequestBuilder {
    id: Option<String>,
}

impl PostV1SalesActsGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesActsGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1SalesActsGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1SalesActsGetRequest, BuildError> {
        Ok(PostV1SalesActsGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

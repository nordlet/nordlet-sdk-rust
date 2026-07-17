pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1EcommerceOrdersGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1EcommerceOrdersGetRequest {
    pub fn builder() -> PostV1EcommerceOrdersGetRequestBuilder {
        <PostV1EcommerceOrdersGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1EcommerceOrdersGetRequestBuilder {
    id: Option<String>,
}

impl PostV1EcommerceOrdersGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1EcommerceOrdersGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1EcommerceOrdersGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1EcommerceOrdersGetRequest, BuildError> {
        Ok(PostV1EcommerceOrdersGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

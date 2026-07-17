pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1EcommerceOrdersCancelRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1EcommerceOrdersCancelRequest {
    pub fn builder() -> PostV1EcommerceOrdersCancelRequestBuilder {
        <PostV1EcommerceOrdersCancelRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1EcommerceOrdersCancelRequestBuilder {
    id: Option<String>,
}

impl PostV1EcommerceOrdersCancelRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1EcommerceOrdersCancelRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1EcommerceOrdersCancelRequestBuilder::id)
    pub fn build(self) -> Result<PostV1EcommerceOrdersCancelRequest, BuildError> {
        Ok(PostV1EcommerceOrdersCancelRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

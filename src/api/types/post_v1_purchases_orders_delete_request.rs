pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PurchasesOrdersDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1PurchasesOrdersDeleteRequest {
    pub fn builder() -> PostV1PurchasesOrdersDeleteRequestBuilder {
        <PostV1PurchasesOrdersDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesOrdersDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1PurchasesOrdersDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesOrdersDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PurchasesOrdersDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1PurchasesOrdersDeleteRequest, BuildError> {
        Ok(PostV1PurchasesOrdersDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

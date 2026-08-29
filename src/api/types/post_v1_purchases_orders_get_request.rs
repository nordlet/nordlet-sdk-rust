pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PurchasesOrdersGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1PurchasesOrdersGetRequest {
    pub fn builder() -> PostV1PurchasesOrdersGetRequestBuilder {
        <PostV1PurchasesOrdersGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesOrdersGetRequestBuilder {
    id: Option<String>,
}

impl PostV1PurchasesOrdersGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesOrdersGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PurchasesOrdersGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1PurchasesOrdersGetRequest, BuildError> {
        Ok(PostV1PurchasesOrdersGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

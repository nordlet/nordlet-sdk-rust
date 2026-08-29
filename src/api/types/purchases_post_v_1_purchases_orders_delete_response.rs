pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PurchasesOrdersDeleteResponse {
    #[serde(default)]
    pub id: String,
}

impl PostV1PurchasesOrdersDeleteResponse {
    pub fn builder() -> PostV1PurchasesOrdersDeleteResponseBuilder {
        <PostV1PurchasesOrdersDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesOrdersDeleteResponseBuilder {
    id: Option<String>,
}

impl PostV1PurchasesOrdersDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesOrdersDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PurchasesOrdersDeleteResponseBuilder::id)
    pub fn build(self) -> Result<PostV1PurchasesOrdersDeleteResponse, BuildError> {
        Ok(PostV1PurchasesOrdersDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

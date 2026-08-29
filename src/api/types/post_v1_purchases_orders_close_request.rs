pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PurchasesOrdersCloseRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl PostV1PurchasesOrdersCloseRequest {
    pub fn builder() -> PostV1PurchasesOrdersCloseRequestBuilder {
        <PostV1PurchasesOrdersCloseRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesOrdersCloseRequestBuilder {
    id: Option<String>,
    reason: Option<String>,
}

impl PostV1PurchasesOrdersCloseRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesOrdersCloseRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PurchasesOrdersCloseRequestBuilder::id)
    pub fn build(self) -> Result<PostV1PurchasesOrdersCloseRequest, BuildError> {
        Ok(PostV1PurchasesOrdersCloseRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            reason: self.reason,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PurchasesOrdersRejectRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl PostV1PurchasesOrdersRejectRequest {
    pub fn builder() -> PostV1PurchasesOrdersRejectRequestBuilder {
        <PostV1PurchasesOrdersRejectRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesOrdersRejectRequestBuilder {
    id: Option<String>,
    reason: Option<String>,
}

impl PostV1PurchasesOrdersRejectRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesOrdersRejectRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PurchasesOrdersRejectRequestBuilder::id)
    pub fn build(self) -> Result<PostV1PurchasesOrdersRejectRequest, BuildError> {
        Ok(PostV1PurchasesOrdersRejectRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            reason: self.reason,
        })
    }
}

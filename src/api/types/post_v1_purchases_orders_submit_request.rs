pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PurchasesOrdersSubmitRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl PostV1PurchasesOrdersSubmitRequest {
    pub fn builder() -> PostV1PurchasesOrdersSubmitRequestBuilder {
        <PostV1PurchasesOrdersSubmitRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesOrdersSubmitRequestBuilder {
    id: Option<String>,
    reason: Option<String>,
}

impl PostV1PurchasesOrdersSubmitRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesOrdersSubmitRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PurchasesOrdersSubmitRequestBuilder::id)
    pub fn build(self) -> Result<PostV1PurchasesOrdersSubmitRequest, BuildError> {
        Ok(PostV1PurchasesOrdersSubmitRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            reason: self.reason,
        })
    }
}

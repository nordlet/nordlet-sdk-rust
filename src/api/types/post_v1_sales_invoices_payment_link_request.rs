pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesPaymentLinkRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1SalesInvoicesPaymentLinkRequest {
    pub fn builder() -> PostV1SalesInvoicesPaymentLinkRequestBuilder {
        <PostV1SalesInvoicesPaymentLinkRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesPaymentLinkRequestBuilder {
    id: Option<String>,
}

impl PostV1SalesInvoicesPaymentLinkRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesPaymentLinkRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1SalesInvoicesPaymentLinkRequestBuilder::id)
    pub fn build(self) -> Result<PostV1SalesInvoicesPaymentLinkRequest, BuildError> {
        Ok(PostV1SalesInvoicesPaymentLinkRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

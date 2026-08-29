pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesEinvoiceSendRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1SalesInvoicesEinvoiceSendRequest {
    pub fn builder() -> PostV1SalesInvoicesEinvoiceSendRequestBuilder {
        <PostV1SalesInvoicesEinvoiceSendRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesEinvoiceSendRequestBuilder {
    id: Option<String>,
}

impl PostV1SalesInvoicesEinvoiceSendRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesEinvoiceSendRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1SalesInvoicesEinvoiceSendRequestBuilder::id)
    pub fn build(self) -> Result<PostV1SalesInvoicesEinvoiceSendRequest, BuildError> {
        Ok(PostV1SalesInvoicesEinvoiceSendRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

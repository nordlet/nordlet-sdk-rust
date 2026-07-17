pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesPeppolSendRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1SalesInvoicesPeppolSendRequest {
    pub fn builder() -> PostV1SalesInvoicesPeppolSendRequestBuilder {
        <PostV1SalesInvoicesPeppolSendRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesPeppolSendRequestBuilder {
    id: Option<String>,
}

impl PostV1SalesInvoicesPeppolSendRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesPeppolSendRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1SalesInvoicesPeppolSendRequestBuilder::id)
    pub fn build(self) -> Result<PostV1SalesInvoicesPeppolSendRequest, BuildError> {
        Ok(PostV1SalesInvoicesPeppolSendRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesEinvoiceStatusRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1SalesInvoicesEinvoiceStatusRequest {
    pub fn builder() -> PostV1SalesInvoicesEinvoiceStatusRequestBuilder {
        <PostV1SalesInvoicesEinvoiceStatusRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesEinvoiceStatusRequestBuilder {
    id: Option<String>,
}

impl PostV1SalesInvoicesEinvoiceStatusRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesEinvoiceStatusRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1SalesInvoicesEinvoiceStatusRequestBuilder::id)
    pub fn build(self) -> Result<PostV1SalesInvoicesEinvoiceStatusRequest, BuildError> {
        Ok(PostV1SalesInvoicesEinvoiceStatusRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

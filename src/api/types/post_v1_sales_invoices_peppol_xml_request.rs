pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesPeppolXmlRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1SalesInvoicesPeppolXmlRequest {
    pub fn builder() -> PostV1SalesInvoicesPeppolXmlRequestBuilder {
        <PostV1SalesInvoicesPeppolXmlRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesPeppolXmlRequestBuilder {
    id: Option<String>,
}

impl PostV1SalesInvoicesPeppolXmlRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesPeppolXmlRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1SalesInvoicesPeppolXmlRequestBuilder::id)
    pub fn build(self) -> Result<PostV1SalesInvoicesPeppolXmlRequest, BuildError> {
        Ok(PostV1SalesInvoicesPeppolXmlRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

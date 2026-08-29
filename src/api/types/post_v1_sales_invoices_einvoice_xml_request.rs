pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesEinvoiceXmlRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1SalesInvoicesEinvoiceXmlRequest {
    pub fn builder() -> PostV1SalesInvoicesEinvoiceXmlRequestBuilder {
        <PostV1SalesInvoicesEinvoiceXmlRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesEinvoiceXmlRequestBuilder {
    id: Option<String>,
}

impl PostV1SalesInvoicesEinvoiceXmlRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesEinvoiceXmlRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1SalesInvoicesEinvoiceXmlRequestBuilder::id)
    pub fn build(self) -> Result<PostV1SalesInvoicesEinvoiceXmlRequest, BuildError> {
        Ok(PostV1SalesInvoicesEinvoiceXmlRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

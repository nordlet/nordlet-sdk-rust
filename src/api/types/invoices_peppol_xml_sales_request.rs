pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesPeppolXmlSalesRequest {
    #[serde(default)]
    pub id: String,
}

impl InvoicesPeppolXmlSalesRequest {
    pub fn builder() -> InvoicesPeppolXmlSalesRequestBuilder {
        <InvoicesPeppolXmlSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesPeppolXmlSalesRequestBuilder {
    id: Option<String>,
}

impl InvoicesPeppolXmlSalesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesPeppolXmlSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoicesPeppolXmlSalesRequestBuilder::id)
    pub fn build(self) -> Result<InvoicesPeppolXmlSalesRequest, BuildError> {
        Ok(InvoicesPeppolXmlSalesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

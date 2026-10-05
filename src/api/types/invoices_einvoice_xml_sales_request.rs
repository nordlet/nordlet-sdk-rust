pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesEinvoiceXmlSalesRequest {
    #[serde(default)]
    pub id: String,
}

impl InvoicesEinvoiceXmlSalesRequest {
    pub fn builder() -> InvoicesEinvoiceXmlSalesRequestBuilder {
        <InvoicesEinvoiceXmlSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesEinvoiceXmlSalesRequestBuilder {
    id: Option<String>,
}

impl InvoicesEinvoiceXmlSalesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesEinvoiceXmlSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoicesEinvoiceXmlSalesRequestBuilder::id)
    pub fn build(self) -> Result<InvoicesEinvoiceXmlSalesRequest, BuildError> {
        Ok(InvoicesEinvoiceXmlSalesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

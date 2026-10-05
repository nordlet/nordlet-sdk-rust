pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesPdfSalesRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<InvoicesPdfSalesRequestLocale>,
}

impl InvoicesPdfSalesRequest {
    pub fn builder() -> InvoicesPdfSalesRequestBuilder {
        <InvoicesPdfSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesPdfSalesRequestBuilder {
    id: Option<String>,
    locale: Option<InvoicesPdfSalesRequestLocale>,
}

impl InvoicesPdfSalesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn locale(mut self, value: InvoicesPdfSalesRequestLocale) -> Self {
        self.locale = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoicesPdfSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoicesPdfSalesRequestBuilder::id)
    pub fn build(self) -> Result<InvoicesPdfSalesRequest, BuildError> {
        Ok(InvoicesPdfSalesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            locale: self.locale,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesSendSalesRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<InvoicesSendSalesRequestLocale>,
}

impl InvoicesSendSalesRequest {
    pub fn builder() -> InvoicesSendSalesRequestBuilder {
        <InvoicesSendSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesSendSalesRequestBuilder {
    id: Option<String>,
    to: Option<String>,
    locale: Option<InvoicesSendSalesRequestLocale>,
}

impl InvoicesSendSalesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.to = Some(value.into());
        self
    }

    pub fn locale(mut self, value: InvoicesSendSalesRequestLocale) -> Self {
        self.locale = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoicesSendSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoicesSendSalesRequestBuilder::id)
    pub fn build(self) -> Result<InvoicesSendSalesRequest, BuildError> {
        Ok(InvoicesSendSalesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            to: self.to,
            locale: self.locale,
        })
    }
}

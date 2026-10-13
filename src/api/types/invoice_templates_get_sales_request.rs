pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoiceTemplatesGetSalesRequest {
    #[serde(default)]
    pub id: String,
}

impl InvoiceTemplatesGetSalesRequest {
    pub fn builder() -> InvoiceTemplatesGetSalesRequestBuilder {
        <InvoiceTemplatesGetSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceTemplatesGetSalesRequestBuilder {
    id: Option<String>,
}

impl InvoiceTemplatesGetSalesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoiceTemplatesGetSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoiceTemplatesGetSalesRequestBuilder::id)
    pub fn build(self) -> Result<InvoiceTemplatesGetSalesRequest, BuildError> {
        Ok(InvoiceTemplatesGetSalesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

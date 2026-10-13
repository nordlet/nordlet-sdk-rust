pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoiceTemplatesDeleteSalesRequest {
    #[serde(default)]
    pub id: String,
}

impl InvoiceTemplatesDeleteSalesRequest {
    pub fn builder() -> InvoiceTemplatesDeleteSalesRequestBuilder {
        <InvoiceTemplatesDeleteSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceTemplatesDeleteSalesRequestBuilder {
    id: Option<String>,
}

impl InvoiceTemplatesDeleteSalesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoiceTemplatesDeleteSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoiceTemplatesDeleteSalesRequestBuilder::id)
    pub fn build(self) -> Result<InvoiceTemplatesDeleteSalesRequest, BuildError> {
        Ok(InvoiceTemplatesDeleteSalesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

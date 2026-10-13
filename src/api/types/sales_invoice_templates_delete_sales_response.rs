pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoiceTemplatesDeleteSalesResponse {
    #[serde(default)]
    pub deleted: bool,
}

impl InvoiceTemplatesDeleteSalesResponse {
    pub fn builder() -> InvoiceTemplatesDeleteSalesResponseBuilder {
        <InvoiceTemplatesDeleteSalesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceTemplatesDeleteSalesResponseBuilder {
    deleted: Option<bool>,
}

impl InvoiceTemplatesDeleteSalesResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoiceTemplatesDeleteSalesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](InvoiceTemplatesDeleteSalesResponseBuilder::deleted)
    pub fn build(self) -> Result<InvoiceTemplatesDeleteSalesResponse, BuildError> {
        Ok(InvoiceTemplatesDeleteSalesResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}

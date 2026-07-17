pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankPaymentsExportRequest {
    #[serde(rename = "bankAccountId")]
    #[serde(default)]
    pub bank_account_id: String,
    #[serde(rename = "purchaseInvoiceIds")]
    #[serde(default)]
    pub purchase_invoice_ids: Vec<String>,
    #[serde(rename = "executionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_date: Option<String>,
}

impl PostV1BankPaymentsExportRequest {
    pub fn builder() -> PostV1BankPaymentsExportRequestBuilder {
        <PostV1BankPaymentsExportRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankPaymentsExportRequestBuilder {
    bank_account_id: Option<String>,
    purchase_invoice_ids: Option<Vec<String>>,
    execution_date: Option<String>,
}

impl PostV1BankPaymentsExportRequestBuilder {
    pub fn bank_account_id(mut self, value: impl Into<String>) -> Self {
        self.bank_account_id = Some(value.into());
        self
    }

    pub fn purchase_invoice_ids(mut self, value: Vec<String>) -> Self {
        self.purchase_invoice_ids = Some(value);
        self
    }

    pub fn execution_date(mut self, value: impl Into<String>) -> Self {
        self.execution_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankPaymentsExportRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bank_account_id`](PostV1BankPaymentsExportRequestBuilder::bank_account_id)
    /// - [`purchase_invoice_ids`](PostV1BankPaymentsExportRequestBuilder::purchase_invoice_ids)
    pub fn build(self) -> Result<PostV1BankPaymentsExportRequest, BuildError> {
        Ok(PostV1BankPaymentsExportRequest {
            bank_account_id: self
                .bank_account_id
                .ok_or_else(|| BuildError::missing_field("bank_account_id"))?,
            purchase_invoice_ids: self
                .purchase_invoice_ids
                .ok_or_else(|| BuildError::missing_field("purchase_invoice_ids"))?,
            execution_date: self.execution_date,
        })
    }
}

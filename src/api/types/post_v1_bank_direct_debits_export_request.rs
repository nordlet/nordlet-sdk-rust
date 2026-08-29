pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankDirectDebitsExportRequest {
    #[serde(rename = "bankAccountId")]
    #[serde(default)]
    pub bank_account_id: String,
    #[serde(rename = "saleInvoiceIds")]
    #[serde(default)]
    pub sale_invoice_ids: Vec<String>,
    #[serde(rename = "collectionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_date: Option<String>,
}

impl PostV1BankDirectDebitsExportRequest {
    pub fn builder() -> PostV1BankDirectDebitsExportRequestBuilder {
        <PostV1BankDirectDebitsExportRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankDirectDebitsExportRequestBuilder {
    bank_account_id: Option<String>,
    sale_invoice_ids: Option<Vec<String>>,
    collection_date: Option<String>,
}

impl PostV1BankDirectDebitsExportRequestBuilder {
    pub fn bank_account_id(mut self, value: impl Into<String>) -> Self {
        self.bank_account_id = Some(value.into());
        self
    }

    pub fn sale_invoice_ids(mut self, value: Vec<String>) -> Self {
        self.sale_invoice_ids = Some(value);
        self
    }

    pub fn collection_date(mut self, value: impl Into<String>) -> Self {
        self.collection_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankDirectDebitsExportRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bank_account_id`](PostV1BankDirectDebitsExportRequestBuilder::bank_account_id)
    /// - [`sale_invoice_ids`](PostV1BankDirectDebitsExportRequestBuilder::sale_invoice_ids)
    pub fn build(self) -> Result<PostV1BankDirectDebitsExportRequest, BuildError> {
        Ok(PostV1BankDirectDebitsExportRequest {
            bank_account_id: self
                .bank_account_id
                .ok_or_else(|| BuildError::missing_field("bank_account_id"))?,
            sale_invoice_ids: self
                .sale_invoice_ids
                .ok_or_else(|| BuildError::missing_field("sale_invoice_ids"))?,
            collection_date: self.collection_date,
        })
    }
}

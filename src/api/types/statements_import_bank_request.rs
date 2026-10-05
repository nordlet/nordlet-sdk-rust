pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StatementsImportBankRequest {
    #[serde(rename = "bankAccountId")]
    #[serde(default)]
    pub bank_account_id: String,
    #[serde(rename = "templateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<StatementsImportBankRequestFormat>,
    #[serde(default)]
    pub content: String,
    /// Stripe transfers export (plain CSV or base64) used to post lender payouts and commissions
    #[serde(rename = "transfersCsv")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfers_csv: Option<String>,
}

impl StatementsImportBankRequest {
    pub fn builder() -> StatementsImportBankRequestBuilder {
        <StatementsImportBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatementsImportBankRequestBuilder {
    bank_account_id: Option<String>,
    template_id: Option<String>,
    format: Option<StatementsImportBankRequestFormat>,
    content: Option<String>,
    transfers_csv: Option<String>,
}

impl StatementsImportBankRequestBuilder {
    pub fn bank_account_id(mut self, value: impl Into<String>) -> Self {
        self.bank_account_id = Some(value.into());
        self
    }

    pub fn template_id(mut self, value: impl Into<String>) -> Self {
        self.template_id = Some(value.into());
        self
    }

    pub fn format(mut self, value: StatementsImportBankRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn transfers_csv(mut self, value: impl Into<String>) -> Self {
        self.transfers_csv = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StatementsImportBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bank_account_id`](StatementsImportBankRequestBuilder::bank_account_id)
    /// - [`content`](StatementsImportBankRequestBuilder::content)
    pub fn build(self) -> Result<StatementsImportBankRequest, BuildError> {
        Ok(StatementsImportBankRequest {
            bank_account_id: self
                .bank_account_id
                .ok_or_else(|| BuildError::missing_field("bank_account_id"))?,
            template_id: self.template_id,
            format: self.format,
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            transfers_csv: self.transfers_csv,
        })
    }
}

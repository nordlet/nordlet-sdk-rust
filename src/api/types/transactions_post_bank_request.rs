pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransactionsPostBankRequest {
    #[serde(rename = "transactionId")]
    #[serde(default)]
    pub transaction_id: String,
    #[serde(rename = "accountCode")]
    #[serde(default)]
    pub account_code: String,
    #[serde(rename = "taxPaymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_payment_id: Option<String>,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[serde(rename = "costCenterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "vatClassifierCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_classifier_code: Option<String>,
}

impl TransactionsPostBankRequest {
    pub fn builder() -> TransactionsPostBankRequestBuilder {
        <TransactionsPostBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionsPostBankRequestBuilder {
    transaction_id: Option<String>,
    account_code: Option<String>,
    tax_payment_id: Option<String>,
    partner_id: Option<String>,
    cost_center_id: Option<String>,
    project_id: Option<String>,
    description: Option<String>,
    vat_classifier_code: Option<String>,
}

impl TransactionsPostBankRequestBuilder {
    pub fn transaction_id(mut self, value: impl Into<String>) -> Self {
        self.transaction_id = Some(value.into());
        self
    }

    pub fn account_code(mut self, value: impl Into<String>) -> Self {
        self.account_code = Some(value.into());
        self
    }

    pub fn tax_payment_id(mut self, value: impl Into<String>) -> Self {
        self.tax_payment_id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn cost_center_id(mut self, value: impl Into<String>) -> Self {
        self.cost_center_id = Some(value.into());
        self
    }

    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn vat_classifier_code(mut self, value: impl Into<String>) -> Self {
        self.vat_classifier_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransactionsPostBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transaction_id`](TransactionsPostBankRequestBuilder::transaction_id)
    /// - [`account_code`](TransactionsPostBankRequestBuilder::account_code)
    pub fn build(self) -> Result<TransactionsPostBankRequest, BuildError> {
        Ok(TransactionsPostBankRequest {
            transaction_id: self
                .transaction_id
                .ok_or_else(|| BuildError::missing_field("transaction_id"))?,
            account_code: self
                .account_code
                .ok_or_else(|| BuildError::missing_field("account_code"))?,
            tax_payment_id: self.tax_payment_id,
            partner_id: self.partner_id,
            cost_center_id: self.cost_center_id,
            project_id: self.project_id,
            description: self.description,
            vat_classifier_code: self.vat_classifier_code,
        })
    }
}

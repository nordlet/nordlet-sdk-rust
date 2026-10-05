pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgreementsBillingRunAgreementsResponseGeneratedItem {
    #[serde(rename = "agreementId")]
    #[serde(default)]
    pub agreement_id: String,
    #[serde(rename = "invoiceId")]
    #[serde(default)]
    pub invoice_id: String,
    #[serde(rename = "periodStart")]
    #[serde(default)]
    pub period_start: String,
    #[serde(rename = "periodEnd")]
    #[serde(default)]
    pub period_end: String,
}

impl AgreementsBillingRunAgreementsResponseGeneratedItem {
    pub fn builder() -> AgreementsBillingRunAgreementsResponseGeneratedItemBuilder {
        <AgreementsBillingRunAgreementsResponseGeneratedItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgreementsBillingRunAgreementsResponseGeneratedItemBuilder {
    agreement_id: Option<String>,
    invoice_id: Option<String>,
    period_start: Option<String>,
    period_end: Option<String>,
}

impl AgreementsBillingRunAgreementsResponseGeneratedItemBuilder {
    pub fn agreement_id(mut self, value: impl Into<String>) -> Self {
        self.agreement_id = Some(value.into());
        self
    }

    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn period_start(mut self, value: impl Into<String>) -> Self {
        self.period_start = Some(value.into());
        self
    }

    pub fn period_end(mut self, value: impl Into<String>) -> Self {
        self.period_end = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgreementsBillingRunAgreementsResponseGeneratedItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agreement_id`](AgreementsBillingRunAgreementsResponseGeneratedItemBuilder::agreement_id)
    /// - [`invoice_id`](AgreementsBillingRunAgreementsResponseGeneratedItemBuilder::invoice_id)
    /// - [`period_start`](AgreementsBillingRunAgreementsResponseGeneratedItemBuilder::period_start)
    /// - [`period_end`](AgreementsBillingRunAgreementsResponseGeneratedItemBuilder::period_end)
    pub fn build(self) -> Result<AgreementsBillingRunAgreementsResponseGeneratedItem, BuildError> {
        Ok(AgreementsBillingRunAgreementsResponseGeneratedItem {
            agreement_id: self
                .agreement_id
                .ok_or_else(|| BuildError::missing_field("agreement_id"))?,
            invoice_id: self
                .invoice_id
                .ok_or_else(|| BuildError::missing_field("invoice_id"))?,
            period_start: self
                .period_start
                .ok_or_else(|| BuildError::missing_field("period_start"))?,
            period_end: self
                .period_end
                .ok_or_else(|| BuildError::missing_field("period_end"))?,
        })
    }
}

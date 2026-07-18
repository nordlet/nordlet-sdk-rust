pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AgreementsAgreementsBillingRunResponseGeneratedItem {
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

impl PostV1AgreementsAgreementsBillingRunResponseGeneratedItem {
    pub fn builder() -> PostV1AgreementsAgreementsBillingRunResponseGeneratedItemBuilder {
        <PostV1AgreementsAgreementsBillingRunResponseGeneratedItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AgreementsAgreementsBillingRunResponseGeneratedItemBuilder {
    agreement_id: Option<String>,
    invoice_id: Option<String>,
    period_start: Option<String>,
    period_end: Option<String>,
}

impl PostV1AgreementsAgreementsBillingRunResponseGeneratedItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1AgreementsAgreementsBillingRunResponseGeneratedItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agreement_id`](PostV1AgreementsAgreementsBillingRunResponseGeneratedItemBuilder::agreement_id)
    /// - [`invoice_id`](PostV1AgreementsAgreementsBillingRunResponseGeneratedItemBuilder::invoice_id)
    /// - [`period_start`](PostV1AgreementsAgreementsBillingRunResponseGeneratedItemBuilder::period_start)
    /// - [`period_end`](PostV1AgreementsAgreementsBillingRunResponseGeneratedItemBuilder::period_end)
    pub fn build(
        self,
    ) -> Result<PostV1AgreementsAgreementsBillingRunResponseGeneratedItem, BuildError> {
        Ok(PostV1AgreementsAgreementsBillingRunResponseGeneratedItem {
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

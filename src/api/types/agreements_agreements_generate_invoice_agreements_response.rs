pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgreementsGenerateInvoiceAgreementsResponse {
    #[serde(rename = "invoiceId")]
    #[serde(default)]
    pub invoice_id: String,
    #[serde(rename = "periodStart")]
    #[serde(default)]
    pub period_start: String,
    #[serde(rename = "periodEnd")]
    #[serde(default)]
    pub period_end: String,
    #[serde(rename = "renewedEndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewed_end_date: Option<NaiveDate>,
}

impl AgreementsGenerateInvoiceAgreementsResponse {
    pub fn builder() -> AgreementsGenerateInvoiceAgreementsResponseBuilder {
        <AgreementsGenerateInvoiceAgreementsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgreementsGenerateInvoiceAgreementsResponseBuilder {
    invoice_id: Option<String>,
    period_start: Option<String>,
    period_end: Option<String>,
    renewed_end_date: Option<NaiveDate>,
}

impl AgreementsGenerateInvoiceAgreementsResponseBuilder {
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

    pub fn renewed_end_date(mut self, value: NaiveDate) -> Self {
        self.renewed_end_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgreementsGenerateInvoiceAgreementsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoice_id`](AgreementsGenerateInvoiceAgreementsResponseBuilder::invoice_id)
    /// - [`period_start`](AgreementsGenerateInvoiceAgreementsResponseBuilder::period_start)
    /// - [`period_end`](AgreementsGenerateInvoiceAgreementsResponseBuilder::period_end)
    pub fn build(self) -> Result<AgreementsGenerateInvoiceAgreementsResponse, BuildError> {
        Ok(AgreementsGenerateInvoiceAgreementsResponse {
            invoice_id: self
                .invoice_id
                .ok_or_else(|| BuildError::missing_field("invoice_id"))?,
            period_start: self
                .period_start
                .ok_or_else(|| BuildError::missing_field("period_start"))?,
            period_end: self
                .period_end
                .ok_or_else(|| BuildError::missing_field("period_end"))?,
            renewed_end_date: self.renewed_end_date,
        })
    }
}

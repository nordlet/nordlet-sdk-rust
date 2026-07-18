pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AgreementsAgreementsGenerateInvoiceResponse {
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
    pub renewed_end_date: Option<String>,
}

impl PostV1AgreementsAgreementsGenerateInvoiceResponse {
    pub fn builder() -> PostV1AgreementsAgreementsGenerateInvoiceResponseBuilder {
        <PostV1AgreementsAgreementsGenerateInvoiceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AgreementsAgreementsGenerateInvoiceResponseBuilder {
    invoice_id: Option<String>,
    period_start: Option<String>,
    period_end: Option<String>,
    renewed_end_date: Option<String>,
}

impl PostV1AgreementsAgreementsGenerateInvoiceResponseBuilder {
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

    pub fn renewed_end_date(mut self, value: impl Into<String>) -> Self {
        self.renewed_end_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AgreementsAgreementsGenerateInvoiceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoice_id`](PostV1AgreementsAgreementsGenerateInvoiceResponseBuilder::invoice_id)
    /// - [`period_start`](PostV1AgreementsAgreementsGenerateInvoiceResponseBuilder::period_start)
    /// - [`period_end`](PostV1AgreementsAgreementsGenerateInvoiceResponseBuilder::period_end)
    pub fn build(self) -> Result<PostV1AgreementsAgreementsGenerateInvoiceResponse, BuildError> {
        Ok(PostV1AgreementsAgreementsGenerateInvoiceResponse {
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

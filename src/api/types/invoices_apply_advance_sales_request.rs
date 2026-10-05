pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesApplyAdvanceSalesRequest {
    #[serde(rename = "advanceId")]
    #[serde(default)]
    pub advance_id: String,
    #[serde(rename = "invoiceId")]
    #[serde(default)]
    pub invoice_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
}

impl InvoicesApplyAdvanceSalesRequest {
    pub fn builder() -> InvoicesApplyAdvanceSalesRequestBuilder {
        <InvoicesApplyAdvanceSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesApplyAdvanceSalesRequestBuilder {
    advance_id: Option<String>,
    invoice_id: Option<String>,
    date: Option<NaiveDate>,
}

impl InvoicesApplyAdvanceSalesRequestBuilder {
    pub fn advance_id(mut self, value: impl Into<String>) -> Self {
        self.advance_id = Some(value.into());
        self
    }

    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoicesApplyAdvanceSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`advance_id`](InvoicesApplyAdvanceSalesRequestBuilder::advance_id)
    /// - [`invoice_id`](InvoicesApplyAdvanceSalesRequestBuilder::invoice_id)
    pub fn build(self) -> Result<InvoicesApplyAdvanceSalesRequest, BuildError> {
        Ok(InvoicesApplyAdvanceSalesRequest {
            advance_id: self
                .advance_id
                .ok_or_else(|| BuildError::missing_field("advance_id"))?,
            invoice_id: self
                .invoice_id
                .ok_or_else(|| BuildError::missing_field("invoice_id"))?,
            date: self.date,
        })
    }
}

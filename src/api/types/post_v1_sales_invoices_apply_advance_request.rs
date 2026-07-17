pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesApplyAdvanceRequest {
    #[serde(rename = "advanceId")]
    #[serde(default)]
    pub advance_id: String,
    #[serde(rename = "invoiceId")]
    #[serde(default)]
    pub invoice_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

impl PostV1SalesInvoicesApplyAdvanceRequest {
    pub fn builder() -> PostV1SalesInvoicesApplyAdvanceRequestBuilder {
        <PostV1SalesInvoicesApplyAdvanceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesApplyAdvanceRequestBuilder {
    advance_id: Option<String>,
    invoice_id: Option<String>,
    date: Option<String>,
}

impl PostV1SalesInvoicesApplyAdvanceRequestBuilder {
    pub fn advance_id(mut self, value: impl Into<String>) -> Self {
        self.advance_id = Some(value.into());
        self
    }

    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesApplyAdvanceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`advance_id`](PostV1SalesInvoicesApplyAdvanceRequestBuilder::advance_id)
    /// - [`invoice_id`](PostV1SalesInvoicesApplyAdvanceRequestBuilder::invoice_id)
    pub fn build(self) -> Result<PostV1SalesInvoicesApplyAdvanceRequest, BuildError> {
        Ok(PostV1SalesInvoicesApplyAdvanceRequest {
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

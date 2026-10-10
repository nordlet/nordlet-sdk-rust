pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InvoicesPeppolStatusSalesResponse {
    #[serde(rename = "messageId")]
    #[serde(default)]
    pub message_id: String,
    pub status: InvoicesPeppolStatusSalesResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(rename = "checkedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub checked_at: DateTime<FixedOffset>,
}

impl InvoicesPeppolStatusSalesResponse {
    pub fn builder() -> InvoicesPeppolStatusSalesResponseBuilder {
        <InvoicesPeppolStatusSalesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesPeppolStatusSalesResponseBuilder {
    message_id: Option<String>,
    status: Option<InvoicesPeppolStatusSalesResponseStatus>,
    detail: Option<String>,
    checked_at: Option<DateTime<FixedOffset>>,
}

impl InvoicesPeppolStatusSalesResponseBuilder {
    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: InvoicesPeppolStatusSalesResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn detail(mut self, value: impl Into<String>) -> Self {
        self.detail = Some(value.into());
        self
    }

    pub fn checked_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.checked_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoicesPeppolStatusSalesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_id`](InvoicesPeppolStatusSalesResponseBuilder::message_id)
    /// - [`status`](InvoicesPeppolStatusSalesResponseBuilder::status)
    /// - [`checked_at`](InvoicesPeppolStatusSalesResponseBuilder::checked_at)
    pub fn build(self) -> Result<InvoicesPeppolStatusSalesResponse, BuildError> {
        Ok(InvoicesPeppolStatusSalesResponse {
            message_id: self
                .message_id
                .ok_or_else(|| BuildError::missing_field("message_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            detail: self.detail,
            checked_at: self
                .checked_at
                .ok_or_else(|| BuildError::missing_field("checked_at"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InvoicesPeppolSendSalesResponse {
    #[serde(default)]
    pub sent: bool,
    #[serde(rename = "messageId")]
    #[serde(default)]
    pub message_id: String,
    #[serde(rename = "receiverId")]
    #[serde(default)]
    pub receiver_id: String,
    pub status: InvoicesPeppolSendSalesResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(rename = "fileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
}

impl InvoicesPeppolSendSalesResponse {
    pub fn builder() -> InvoicesPeppolSendSalesResponseBuilder {
        <InvoicesPeppolSendSalesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesPeppolSendSalesResponseBuilder {
    sent: Option<bool>,
    message_id: Option<String>,
    receiver_id: Option<String>,
    status: Option<InvoicesPeppolSendSalesResponseStatus>,
    detail: Option<String>,
    file_id: Option<String>,
}

impl InvoicesPeppolSendSalesResponseBuilder {
    pub fn sent(mut self, value: bool) -> Self {
        self.sent = Some(value);
        self
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn receiver_id(mut self, value: impl Into<String>) -> Self {
        self.receiver_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: InvoicesPeppolSendSalesResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn detail(mut self, value: impl Into<String>) -> Self {
        self.detail = Some(value.into());
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesPeppolSendSalesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sent`](InvoicesPeppolSendSalesResponseBuilder::sent)
    /// - [`message_id`](InvoicesPeppolSendSalesResponseBuilder::message_id)
    /// - [`receiver_id`](InvoicesPeppolSendSalesResponseBuilder::receiver_id)
    /// - [`status`](InvoicesPeppolSendSalesResponseBuilder::status)
    pub fn build(self) -> Result<InvoicesPeppolSendSalesResponse, BuildError> {
        Ok(InvoicesPeppolSendSalesResponse {
            sent: self.sent.ok_or_else(|| BuildError::missing_field("sent"))?,
            message_id: self
                .message_id
                .ok_or_else(|| BuildError::missing_field("message_id"))?,
            receiver_id: self
                .receiver_id
                .ok_or_else(|| BuildError::missing_field("receiver_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            detail: self.detail,
            file_id: self.file_id,
        })
    }
}

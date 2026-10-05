pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InvoicesEinvoiceSendSalesResponse {
    #[serde(default)]
    pub sent: bool,
    #[serde(default)]
    pub system: String,
    #[serde(default)]
    pub format: String,
    pub transport: InvoicesEinvoiceSendSalesResponseTransport,
    #[serde(rename = "messageId")]
    #[serde(default)]
    pub message_id: String,
    #[serde(rename = "nationalNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_number: Option<String>,
    pub status: InvoicesEinvoiceSendSalesResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(rename = "fileId")]
    #[serde(default)]
    pub file_id: String,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl InvoicesEinvoiceSendSalesResponse {
    pub fn builder() -> InvoicesEinvoiceSendSalesResponseBuilder {
        <InvoicesEinvoiceSendSalesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesEinvoiceSendSalesResponseBuilder {
    sent: Option<bool>,
    system: Option<String>,
    format: Option<String>,
    transport: Option<InvoicesEinvoiceSendSalesResponseTransport>,
    message_id: Option<String>,
    national_number: Option<String>,
    status: Option<InvoicesEinvoiceSendSalesResponseStatus>,
    detail: Option<String>,
    file_id: Option<String>,
    warnings: Option<Vec<String>>,
}

impl InvoicesEinvoiceSendSalesResponseBuilder {
    pub fn sent(mut self, value: bool) -> Self {
        self.sent = Some(value);
        self
    }

    pub fn system(mut self, value: impl Into<String>) -> Self {
        self.system = Some(value.into());
        self
    }

    pub fn format(mut self, value: impl Into<String>) -> Self {
        self.format = Some(value.into());
        self
    }

    pub fn transport(mut self, value: InvoicesEinvoiceSendSalesResponseTransport) -> Self {
        self.transport = Some(value);
        self
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn national_number(mut self, value: impl Into<String>) -> Self {
        self.national_number = Some(value.into());
        self
    }

    pub fn status(mut self, value: InvoicesEinvoiceSendSalesResponseStatus) -> Self {
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

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoicesEinvoiceSendSalesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sent`](InvoicesEinvoiceSendSalesResponseBuilder::sent)
    /// - [`system`](InvoicesEinvoiceSendSalesResponseBuilder::system)
    /// - [`format`](InvoicesEinvoiceSendSalesResponseBuilder::format)
    /// - [`transport`](InvoicesEinvoiceSendSalesResponseBuilder::transport)
    /// - [`message_id`](InvoicesEinvoiceSendSalesResponseBuilder::message_id)
    /// - [`status`](InvoicesEinvoiceSendSalesResponseBuilder::status)
    /// - [`file_id`](InvoicesEinvoiceSendSalesResponseBuilder::file_id)
    /// - [`warnings`](InvoicesEinvoiceSendSalesResponseBuilder::warnings)
    pub fn build(self) -> Result<InvoicesEinvoiceSendSalesResponse, BuildError> {
        Ok(InvoicesEinvoiceSendSalesResponse {
            sent: self.sent.ok_or_else(|| BuildError::missing_field("sent"))?,
            system: self
                .system
                .ok_or_else(|| BuildError::missing_field("system"))?,
            format: self
                .format
                .ok_or_else(|| BuildError::missing_field("format"))?,
            transport: self
                .transport
                .ok_or_else(|| BuildError::missing_field("transport"))?,
            message_id: self
                .message_id
                .ok_or_else(|| BuildError::missing_field("message_id"))?,
            national_number: self.national_number,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            detail: self.detail,
            file_id: self
                .file_id
                .ok_or_else(|| BuildError::missing_field("file_id"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
        })
    }
}

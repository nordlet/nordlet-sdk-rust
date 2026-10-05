pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InvoicesEinvoiceStatusSalesResponse {
    #[serde(default)]
    pub system: String,
    pub transport: InvoicesEinvoiceStatusSalesResponseTransport,
    #[serde(rename = "messageId")]
    #[serde(default)]
    pub message_id: String,
    #[serde(rename = "nationalNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_number: Option<String>,
    pub status: InvoicesEinvoiceStatusSalesResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl InvoicesEinvoiceStatusSalesResponse {
    pub fn builder() -> InvoicesEinvoiceStatusSalesResponseBuilder {
        <InvoicesEinvoiceStatusSalesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesEinvoiceStatusSalesResponseBuilder {
    system: Option<String>,
    transport: Option<InvoicesEinvoiceStatusSalesResponseTransport>,
    message_id: Option<String>,
    national_number: Option<String>,
    status: Option<InvoicesEinvoiceStatusSalesResponseStatus>,
    detail: Option<String>,
}

impl InvoicesEinvoiceStatusSalesResponseBuilder {
    pub fn system(mut self, value: impl Into<String>) -> Self {
        self.system = Some(value.into());
        self
    }

    pub fn transport(mut self, value: InvoicesEinvoiceStatusSalesResponseTransport) -> Self {
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

    pub fn status(mut self, value: InvoicesEinvoiceStatusSalesResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn detail(mut self, value: impl Into<String>) -> Self {
        self.detail = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesEinvoiceStatusSalesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`system`](InvoicesEinvoiceStatusSalesResponseBuilder::system)
    /// - [`transport`](InvoicesEinvoiceStatusSalesResponseBuilder::transport)
    /// - [`message_id`](InvoicesEinvoiceStatusSalesResponseBuilder::message_id)
    /// - [`status`](InvoicesEinvoiceStatusSalesResponseBuilder::status)
    pub fn build(self) -> Result<InvoicesEinvoiceStatusSalesResponse, BuildError> {
        Ok(InvoicesEinvoiceStatusSalesResponse {
            system: self
                .system
                .ok_or_else(|| BuildError::missing_field("system"))?,
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
        })
    }
}

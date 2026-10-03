pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesEinvoiceStatusResponse {
    #[serde(default)]
    pub system: String,
    pub transport: PostV1SalesInvoicesEinvoiceStatusResponseTransport,
    #[serde(rename = "messageId")]
    #[serde(default)]
    pub message_id: String,
    #[serde(rename = "nationalNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_number: Option<String>,
    pub status: PostV1SalesInvoicesEinvoiceStatusResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl PostV1SalesInvoicesEinvoiceStatusResponse {
    pub fn builder() -> PostV1SalesInvoicesEinvoiceStatusResponseBuilder {
        <PostV1SalesInvoicesEinvoiceStatusResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesEinvoiceStatusResponseBuilder {
    system: Option<String>,
    transport: Option<PostV1SalesInvoicesEinvoiceStatusResponseTransport>,
    message_id: Option<String>,
    national_number: Option<String>,
    status: Option<PostV1SalesInvoicesEinvoiceStatusResponseStatus>,
    detail: Option<String>,
}

impl PostV1SalesInvoicesEinvoiceStatusResponseBuilder {
    pub fn system(mut self, value: impl Into<String>) -> Self {
        self.system = Some(value.into());
        self
    }

    pub fn transport(mut self, value: PostV1SalesInvoicesEinvoiceStatusResponseTransport) -> Self {
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

    pub fn status(mut self, value: PostV1SalesInvoicesEinvoiceStatusResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn detail(mut self, value: impl Into<String>) -> Self {
        self.detail = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesEinvoiceStatusResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`system`](PostV1SalesInvoicesEinvoiceStatusResponseBuilder::system)
    /// - [`transport`](PostV1SalesInvoicesEinvoiceStatusResponseBuilder::transport)
    /// - [`message_id`](PostV1SalesInvoicesEinvoiceStatusResponseBuilder::message_id)
    /// - [`status`](PostV1SalesInvoicesEinvoiceStatusResponseBuilder::status)
    pub fn build(self) -> Result<PostV1SalesInvoicesEinvoiceStatusResponse, BuildError> {
        Ok(PostV1SalesInvoicesEinvoiceStatusResponse {
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

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PaymentsExportBankResponse {
    pub format: PaymentsExportBankResponseFormat,
    #[serde(rename = "messageId")]
    #[serde(default)]
    pub message_id: String,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "contentType")]
    #[serde(default)]
    pub content_type: String,
    #[serde(rename = "transactionCount")]
    #[serde(default)]
    pub transaction_count: i64,
    #[serde(rename = "controlSum")]
    #[serde(default)]
    pub control_sum: String,
    #[serde(default)]
    pub content: String,
}

impl PaymentsExportBankResponse {
    pub fn builder() -> PaymentsExportBankResponseBuilder {
        <PaymentsExportBankResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentsExportBankResponseBuilder {
    format: Option<PaymentsExportBankResponseFormat>,
    message_id: Option<String>,
    file_name: Option<String>,
    content_type: Option<String>,
    transaction_count: Option<i64>,
    control_sum: Option<String>,
    content: Option<String>,
}

impl PaymentsExportBankResponseBuilder {
    pub fn format(mut self, value: PaymentsExportBankResponseFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn transaction_count(mut self, value: i64) -> Self {
        self.transaction_count = Some(value);
        self
    }

    pub fn control_sum(mut self, value: impl Into<String>) -> Self {
        self.control_sum = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentsExportBankResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`format`](PaymentsExportBankResponseBuilder::format)
    /// - [`message_id`](PaymentsExportBankResponseBuilder::message_id)
    /// - [`file_name`](PaymentsExportBankResponseBuilder::file_name)
    /// - [`content_type`](PaymentsExportBankResponseBuilder::content_type)
    /// - [`transaction_count`](PaymentsExportBankResponseBuilder::transaction_count)
    /// - [`control_sum`](PaymentsExportBankResponseBuilder::control_sum)
    /// - [`content`](PaymentsExportBankResponseBuilder::content)
    pub fn build(self) -> Result<PaymentsExportBankResponse, BuildError> {
        Ok(PaymentsExportBankResponse {
            format: self
                .format
                .ok_or_else(|| BuildError::missing_field("format"))?,
            message_id: self
                .message_id
                .ok_or_else(|| BuildError::missing_field("message_id"))?,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            content_type: self
                .content_type
                .ok_or_else(|| BuildError::missing_field("content_type"))?,
            transaction_count: self
                .transaction_count
                .ok_or_else(|| BuildError::missing_field("transaction_count"))?,
            control_sum: self
                .control_sum
                .ok_or_else(|| BuildError::missing_field("control_sum"))?,
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MandatesAuddisExportBankResponse {
    pub format: MandatesAuddisExportBankResponseFormat,
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
    #[serde(rename = "processingDate")]
    #[serde(default)]
    pub processing_date: NaiveDate,
    #[serde(default)]
    pub lodged: Vec<String>,
    #[serde(default)]
    pub cancelled: Vec<String>,
}

impl MandatesAuddisExportBankResponse {
    pub fn builder() -> MandatesAuddisExportBankResponseBuilder {
        <MandatesAuddisExportBankResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MandatesAuddisExportBankResponseBuilder {
    format: Option<MandatesAuddisExportBankResponseFormat>,
    message_id: Option<String>,
    file_name: Option<String>,
    content_type: Option<String>,
    transaction_count: Option<i64>,
    control_sum: Option<String>,
    content: Option<String>,
    processing_date: Option<NaiveDate>,
    lodged: Option<Vec<String>>,
    cancelled: Option<Vec<String>>,
}

impl MandatesAuddisExportBankResponseBuilder {
    pub fn format(mut self, value: MandatesAuddisExportBankResponseFormat) -> Self {
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

    pub fn processing_date(mut self, value: NaiveDate) -> Self {
        self.processing_date = Some(value);
        self
    }

    pub fn lodged(mut self, value: Vec<String>) -> Self {
        self.lodged = Some(value);
        self
    }

    pub fn cancelled(mut self, value: Vec<String>) -> Self {
        self.cancelled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MandatesAuddisExportBankResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`format`](MandatesAuddisExportBankResponseBuilder::format)
    /// - [`message_id`](MandatesAuddisExportBankResponseBuilder::message_id)
    /// - [`file_name`](MandatesAuddisExportBankResponseBuilder::file_name)
    /// - [`content_type`](MandatesAuddisExportBankResponseBuilder::content_type)
    /// - [`transaction_count`](MandatesAuddisExportBankResponseBuilder::transaction_count)
    /// - [`control_sum`](MandatesAuddisExportBankResponseBuilder::control_sum)
    /// - [`content`](MandatesAuddisExportBankResponseBuilder::content)
    /// - [`processing_date`](MandatesAuddisExportBankResponseBuilder::processing_date)
    /// - [`lodged`](MandatesAuddisExportBankResponseBuilder::lodged)
    /// - [`cancelled`](MandatesAuddisExportBankResponseBuilder::cancelled)
    pub fn build(self) -> Result<MandatesAuddisExportBankResponse, BuildError> {
        Ok(MandatesAuddisExportBankResponse {
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
            processing_date: self
                .processing_date
                .ok_or_else(|| BuildError::missing_field("processing_date"))?,
            lodged: self
                .lodged
                .ok_or_else(|| BuildError::missing_field("lodged"))?,
            cancelled: self
                .cancelled
                .ok_or_else(|| BuildError::missing_field("cancelled"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DirectDebitsExportBankResponse {
    pub format: DirectDebitsExportBankResponseFormat,
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
    #[serde(rename = "collectionDate")]
    #[serde(default)]
    pub collection_date: NaiveDate,
}

impl DirectDebitsExportBankResponse {
    pub fn builder() -> DirectDebitsExportBankResponseBuilder {
        <DirectDebitsExportBankResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DirectDebitsExportBankResponseBuilder {
    format: Option<DirectDebitsExportBankResponseFormat>,
    message_id: Option<String>,
    file_name: Option<String>,
    content_type: Option<String>,
    transaction_count: Option<i64>,
    control_sum: Option<String>,
    content: Option<String>,
    collection_date: Option<NaiveDate>,
}

impl DirectDebitsExportBankResponseBuilder {
    pub fn format(mut self, value: DirectDebitsExportBankResponseFormat) -> Self {
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

    pub fn collection_date(mut self, value: NaiveDate) -> Self {
        self.collection_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DirectDebitsExportBankResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`format`](DirectDebitsExportBankResponseBuilder::format)
    /// - [`message_id`](DirectDebitsExportBankResponseBuilder::message_id)
    /// - [`file_name`](DirectDebitsExportBankResponseBuilder::file_name)
    /// - [`content_type`](DirectDebitsExportBankResponseBuilder::content_type)
    /// - [`transaction_count`](DirectDebitsExportBankResponseBuilder::transaction_count)
    /// - [`control_sum`](DirectDebitsExportBankResponseBuilder::control_sum)
    /// - [`content`](DirectDebitsExportBankResponseBuilder::content)
    /// - [`collection_date`](DirectDebitsExportBankResponseBuilder::collection_date)
    pub fn build(self) -> Result<DirectDebitsExportBankResponse, BuildError> {
        Ok(DirectDebitsExportBankResponse {
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
            collection_date: self
                .collection_date
                .ok_or_else(|| BuildError::missing_field("collection_date"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollPaymentsExportResponse {
    #[serde(rename = "messageId")]
    #[serde(default)]
    pub message_id: String,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "transactionCount")]
    #[serde(default)]
    pub transaction_count: i64,
    #[serde(rename = "controlSum")]
    #[serde(default)]
    pub control_sum: String,
    #[serde(default)]
    pub xml: String,
}

impl PostV1PayrollPaymentsExportResponse {
    pub fn builder() -> PostV1PayrollPaymentsExportResponseBuilder {
        <PostV1PayrollPaymentsExportResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollPaymentsExportResponseBuilder {
    message_id: Option<String>,
    file_name: Option<String>,
    transaction_count: Option<i64>,
    control_sum: Option<String>,
    xml: Option<String>,
}

impl PostV1PayrollPaymentsExportResponseBuilder {
    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
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

    pub fn xml(mut self, value: impl Into<String>) -> Self {
        self.xml = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PayrollPaymentsExportResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_id`](PostV1PayrollPaymentsExportResponseBuilder::message_id)
    /// - [`file_name`](PostV1PayrollPaymentsExportResponseBuilder::file_name)
    /// - [`transaction_count`](PostV1PayrollPaymentsExportResponseBuilder::transaction_count)
    /// - [`control_sum`](PostV1PayrollPaymentsExportResponseBuilder::control_sum)
    /// - [`xml`](PostV1PayrollPaymentsExportResponseBuilder::xml)
    pub fn build(self) -> Result<PostV1PayrollPaymentsExportResponse, BuildError> {
        Ok(PostV1PayrollPaymentsExportResponse {
            message_id: self
                .message_id
                .ok_or_else(|| BuildError::missing_field("message_id"))?,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            transaction_count: self
                .transaction_count
                .ok_or_else(|| BuildError::missing_field("transaction_count"))?,
            control_sum: self
                .control_sum
                .ok_or_else(|| BuildError::missing_field("control_sum"))?,
            xml: self.xml.ok_or_else(|| BuildError::missing_field("xml"))?,
        })
    }
}

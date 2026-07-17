pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollPaymentsExportRequest {
    #[serde(rename = "runId")]
    #[serde(default)]
    pub run_id: String,
    #[serde(rename = "bankAccountId")]
    #[serde(default)]
    pub bank_account_id: String,
    #[serde(rename = "executionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_date: Option<String>,
}

impl PostV1PayrollPaymentsExportRequest {
    pub fn builder() -> PostV1PayrollPaymentsExportRequestBuilder {
        <PostV1PayrollPaymentsExportRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollPaymentsExportRequestBuilder {
    run_id: Option<String>,
    bank_account_id: Option<String>,
    execution_date: Option<String>,
}

impl PostV1PayrollPaymentsExportRequestBuilder {
    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn bank_account_id(mut self, value: impl Into<String>) -> Self {
        self.bank_account_id = Some(value.into());
        self
    }

    pub fn execution_date(mut self, value: impl Into<String>) -> Self {
        self.execution_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PayrollPaymentsExportRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`run_id`](PostV1PayrollPaymentsExportRequestBuilder::run_id)
    /// - [`bank_account_id`](PostV1PayrollPaymentsExportRequestBuilder::bank_account_id)
    pub fn build(self) -> Result<PostV1PayrollPaymentsExportRequest, BuildError> {
        Ok(PostV1PayrollPaymentsExportRequest {
            run_id: self
                .run_id
                .ok_or_else(|| BuildError::missing_field("run_id"))?,
            bank_account_id: self
                .bank_account_id
                .ok_or_else(|| BuildError::missing_field("bank_account_id"))?,
            execution_date: self.execution_date,
        })
    }
}

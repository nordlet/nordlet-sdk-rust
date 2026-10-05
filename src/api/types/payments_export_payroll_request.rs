pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentsExportPayrollRequest {
    #[serde(rename = "runId")]
    #[serde(default)]
    pub run_id: String,
    #[serde(rename = "bankAccountId")]
    #[serde(default)]
    pub bank_account_id: String,
    #[serde(rename = "executionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<PaymentsExportPayrollRequestLocale>,
}

impl PaymentsExportPayrollRequest {
    pub fn builder() -> PaymentsExportPayrollRequestBuilder {
        <PaymentsExportPayrollRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentsExportPayrollRequestBuilder {
    run_id: Option<String>,
    bank_account_id: Option<String>,
    execution_date: Option<NaiveDate>,
    locale: Option<PaymentsExportPayrollRequestLocale>,
}

impl PaymentsExportPayrollRequestBuilder {
    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn bank_account_id(mut self, value: impl Into<String>) -> Self {
        self.bank_account_id = Some(value.into());
        self
    }

    pub fn execution_date(mut self, value: NaiveDate) -> Self {
        self.execution_date = Some(value);
        self
    }

    pub fn locale(mut self, value: PaymentsExportPayrollRequestLocale) -> Self {
        self.locale = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentsExportPayrollRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`run_id`](PaymentsExportPayrollRequestBuilder::run_id)
    /// - [`bank_account_id`](PaymentsExportPayrollRequestBuilder::bank_account_id)
    pub fn build(self) -> Result<PaymentsExportPayrollRequest, BuildError> {
        Ok(PaymentsExportPayrollRequest {
            run_id: self
                .run_id
                .ok_or_else(|| BuildError::missing_field("run_id"))?,
            bank_account_id: self
                .bank_account_id
                .ok_or_else(|| BuildError::missing_field("bank_account_id"))?,
            execution_date: self.execution_date,
            locale: self.locale,
        })
    }
}

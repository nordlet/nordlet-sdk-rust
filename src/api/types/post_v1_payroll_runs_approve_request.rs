pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollRunsApproveRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "wageAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wage_account_code: Option<String>,
    #[serde(rename = "employerAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employer_account_code: Option<String>,
    #[serde(rename = "payableAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payable_account_code: Option<String>,
    #[serde(rename = "gpmAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpm_account_code: Option<String>,
    #[serde(rename = "sodraAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sodra_account_code: Option<String>,
    #[serde(rename = "deductionAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deduction_account_code: Option<String>,
}

impl PostV1PayrollRunsApproveRequest {
    pub fn builder() -> PostV1PayrollRunsApproveRequestBuilder {
        <PostV1PayrollRunsApproveRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollRunsApproveRequestBuilder {
    id: Option<String>,
    wage_account_code: Option<String>,
    employer_account_code: Option<String>,
    payable_account_code: Option<String>,
    gpm_account_code: Option<String>,
    sodra_account_code: Option<String>,
    deduction_account_code: Option<String>,
}

impl PostV1PayrollRunsApproveRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn wage_account_code(mut self, value: impl Into<String>) -> Self {
        self.wage_account_code = Some(value.into());
        self
    }

    pub fn employer_account_code(mut self, value: impl Into<String>) -> Self {
        self.employer_account_code = Some(value.into());
        self
    }

    pub fn payable_account_code(mut self, value: impl Into<String>) -> Self {
        self.payable_account_code = Some(value.into());
        self
    }

    pub fn gpm_account_code(mut self, value: impl Into<String>) -> Self {
        self.gpm_account_code = Some(value.into());
        self
    }

    pub fn sodra_account_code(mut self, value: impl Into<String>) -> Self {
        self.sodra_account_code = Some(value.into());
        self
    }

    pub fn deduction_account_code(mut self, value: impl Into<String>) -> Self {
        self.deduction_account_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PayrollRunsApproveRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PayrollRunsApproveRequestBuilder::id)
    pub fn build(self) -> Result<PostV1PayrollRunsApproveRequest, BuildError> {
        Ok(PostV1PayrollRunsApproveRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            wage_account_code: self.wage_account_code,
            employer_account_code: self.employer_account_code,
            payable_account_code: self.payable_account_code,
            gpm_account_code: self.gpm_account_code,
            sodra_account_code: self.sodra_account_code,
            deduction_account_code: self.deduction_account_code,
        })
    }
}

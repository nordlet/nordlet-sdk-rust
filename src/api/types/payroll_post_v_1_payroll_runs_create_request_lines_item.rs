pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollRunsCreateRequestLinesItem {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additions: Option<Vec<PostV1PayrollRunsCreateRequestLinesItemAdditionsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deductions: Option<Vec<PostV1PayrollRunsCreateRequestLinesItemDeductionsItem>>,
}

impl PostV1PayrollRunsCreateRequestLinesItem {
    pub fn builder() -> PostV1PayrollRunsCreateRequestLinesItemBuilder {
        <PostV1PayrollRunsCreateRequestLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollRunsCreateRequestLinesItemBuilder {
    employee_id: Option<String>,
    gross: Option<String>,
    additions: Option<Vec<PostV1PayrollRunsCreateRequestLinesItemAdditionsItem>>,
    deductions: Option<Vec<PostV1PayrollRunsCreateRequestLinesItemDeductionsItem>>,
}

impl PostV1PayrollRunsCreateRequestLinesItemBuilder {
    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn gross(mut self, value: impl Into<String>) -> Self {
        self.gross = Some(value.into());
        self
    }

    pub fn additions(
        mut self,
        value: Vec<PostV1PayrollRunsCreateRequestLinesItemAdditionsItem>,
    ) -> Self {
        self.additions = Some(value);
        self
    }

    pub fn deductions(
        mut self,
        value: Vec<PostV1PayrollRunsCreateRequestLinesItemDeductionsItem>,
    ) -> Self {
        self.deductions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PayrollRunsCreateRequestLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](PostV1PayrollRunsCreateRequestLinesItemBuilder::employee_id)
    pub fn build(self) -> Result<PostV1PayrollRunsCreateRequestLinesItem, BuildError> {
        Ok(PostV1PayrollRunsCreateRequestLinesItem {
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            gross: self.gross,
            additions: self.additions,
            deductions: self.deductions,
        })
    }
}

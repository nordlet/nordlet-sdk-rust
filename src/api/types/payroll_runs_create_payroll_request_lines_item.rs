pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RunsCreatePayrollRequestLinesItem {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additions: Option<Vec<RunsCreatePayrollRequestLinesItemAdditionsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deductions: Option<Vec<RunsCreatePayrollRequestLinesItemDeductionsItem>>,
}

impl RunsCreatePayrollRequestLinesItem {
    pub fn builder() -> RunsCreatePayrollRequestLinesItemBuilder {
        <RunsCreatePayrollRequestLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunsCreatePayrollRequestLinesItemBuilder {
    employee_id: Option<String>,
    gross: Option<String>,
    additions: Option<Vec<RunsCreatePayrollRequestLinesItemAdditionsItem>>,
    deductions: Option<Vec<RunsCreatePayrollRequestLinesItemDeductionsItem>>,
}

impl RunsCreatePayrollRequestLinesItemBuilder {
    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn gross(mut self, value: impl Into<String>) -> Self {
        self.gross = Some(value.into());
        self
    }

    pub fn additions(mut self, value: Vec<RunsCreatePayrollRequestLinesItemAdditionsItem>) -> Self {
        self.additions = Some(value);
        self
    }

    pub fn deductions(
        mut self,
        value: Vec<RunsCreatePayrollRequestLinesItemDeductionsItem>,
    ) -> Self {
        self.deductions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RunsCreatePayrollRequestLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](RunsCreatePayrollRequestLinesItemBuilder::employee_id)
    pub fn build(self) -> Result<RunsCreatePayrollRequestLinesItem, BuildError> {
        Ok(RunsCreatePayrollRequestLinesItem {
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            gross: self.gross,
            additions: self.additions,
            deductions: self.deductions,
        })
    }
}

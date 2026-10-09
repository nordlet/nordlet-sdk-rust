pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExpenseReportsCreateCashRequest {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default)]
    pub lines: Vec<ExpenseReportsCreateCashRequestLinesItem>,
}

impl ExpenseReportsCreateCashRequest {
    pub fn builder() -> ExpenseReportsCreateCashRequestBuilder {
        <ExpenseReportsCreateCashRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExpenseReportsCreateCashRequestBuilder {
    employee_id: Option<String>,
    date: Option<NaiveDate>,
    notes: Option<String>,
    lines: Option<Vec<ExpenseReportsCreateCashRequestLinesItem>>,
}

impl ExpenseReportsCreateCashRequestBuilder {
    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn lines(mut self, value: Vec<ExpenseReportsCreateCashRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExpenseReportsCreateCashRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](ExpenseReportsCreateCashRequestBuilder::employee_id)
    /// - [`date`](ExpenseReportsCreateCashRequestBuilder::date)
    /// - [`lines`](ExpenseReportsCreateCashRequestBuilder::lines)
    pub fn build(self) -> Result<ExpenseReportsCreateCashRequest, BuildError> {
        Ok(ExpenseReportsCreateCashRequest {
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            notes: self.notes,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RunsCreatePayrollRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<RunsCreatePayrollRequestFrequency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<i64>,
    #[serde(rename = "periodStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_start: Option<NaiveDate>,
    #[serde(rename = "includeNatura")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_natura: Option<bool>,
    #[serde(rename = "grossOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_overrides: Option<Vec<RunsCreatePayrollRequestGrossOverridesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<RunsCreatePayrollRequestLinesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "payDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_date: Option<NaiveDate>,
}

impl RunsCreatePayrollRequest {
    pub fn builder() -> RunsCreatePayrollRequestBuilder {
        <RunsCreatePayrollRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunsCreatePayrollRequestBuilder {
    frequency: Option<RunsCreatePayrollRequestFrequency>,
    year: Option<i64>,
    month: Option<i64>,
    period_start: Option<NaiveDate>,
    include_natura: Option<bool>,
    gross_overrides: Option<Vec<RunsCreatePayrollRequestGrossOverridesItem>>,
    lines: Option<Vec<RunsCreatePayrollRequestLinesItem>>,
    notes: Option<String>,
    pay_date: Option<NaiveDate>,
}

impl RunsCreatePayrollRequestBuilder {
    pub fn frequency(mut self, value: RunsCreatePayrollRequestFrequency) -> Self {
        self.frequency = Some(value);
        self
    }

    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn period_start(mut self, value: NaiveDate) -> Self {
        self.period_start = Some(value);
        self
    }

    pub fn include_natura(mut self, value: bool) -> Self {
        self.include_natura = Some(value);
        self
    }

    pub fn gross_overrides(
        mut self,
        value: Vec<RunsCreatePayrollRequestGrossOverridesItem>,
    ) -> Self {
        self.gross_overrides = Some(value);
        self
    }

    pub fn lines(mut self, value: Vec<RunsCreatePayrollRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn pay_date(mut self, value: NaiveDate) -> Self {
        self.pay_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RunsCreatePayrollRequest`].
    pub fn build(self) -> Result<RunsCreatePayrollRequest, BuildError> {
        Ok(RunsCreatePayrollRequest {
            frequency: self.frequency,
            year: self.year,
            month: self.month,
            period_start: self.period_start,
            include_natura: self.include_natura,
            gross_overrides: self.gross_overrides,
            lines: self.lines,
            notes: self.notes,
            pay_date: self.pay_date,
        })
    }
}

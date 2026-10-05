pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SchedulesListPayrollResponse {
    #[serde(default)]
    pub rows: Vec<SchedulesListPayrollResponseRowsItem>,
}

impl SchedulesListPayrollResponse {
    pub fn builder() -> SchedulesListPayrollResponseBuilder {
        <SchedulesListPayrollResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SchedulesListPayrollResponseBuilder {
    rows: Option<Vec<SchedulesListPayrollResponseRowsItem>>,
}

impl SchedulesListPayrollResponseBuilder {
    pub fn rows(mut self, value: Vec<SchedulesListPayrollResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SchedulesListPayrollResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](SchedulesListPayrollResponseBuilder::rows)
    pub fn build(self) -> Result<SchedulesListPayrollResponse, BuildError> {
        Ok(SchedulesListPayrollResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}

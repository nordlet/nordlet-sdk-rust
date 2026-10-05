pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DepartmentsListPayrollResponse {
    #[serde(default)]
    pub rows: Vec<DepartmentsListPayrollResponseRowsItem>,
}

impl DepartmentsListPayrollResponse {
    pub fn builder() -> DepartmentsListPayrollResponseBuilder {
        <DepartmentsListPayrollResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DepartmentsListPayrollResponseBuilder {
    rows: Option<Vec<DepartmentsListPayrollResponseRowsItem>>,
}

impl DepartmentsListPayrollResponseBuilder {
    pub fn rows(mut self, value: Vec<DepartmentsListPayrollResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DepartmentsListPayrollResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](DepartmentsListPayrollResponseBuilder::rows)
    pub fn build(self) -> Result<DepartmentsListPayrollResponse, BuildError> {
        Ok(DepartmentsListPayrollResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}

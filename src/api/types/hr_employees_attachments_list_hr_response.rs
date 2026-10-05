pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmployeesAttachmentsListHrResponse {
    #[serde(default)]
    pub rows: Vec<EmployeesAttachmentsListHrResponseRowsItem>,
}

impl EmployeesAttachmentsListHrResponse {
    pub fn builder() -> EmployeesAttachmentsListHrResponseBuilder {
        <EmployeesAttachmentsListHrResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmployeesAttachmentsListHrResponseBuilder {
    rows: Option<Vec<EmployeesAttachmentsListHrResponseRowsItem>>,
}

impl EmployeesAttachmentsListHrResponseBuilder {
    pub fn rows(mut self, value: Vec<EmployeesAttachmentsListHrResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EmployeesAttachmentsListHrResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](EmployeesAttachmentsListHrResponseBuilder::rows)
    pub fn build(self) -> Result<EmployeesAttachmentsListHrResponse, BuildError> {
        Ok(EmployeesAttachmentsListHrResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}

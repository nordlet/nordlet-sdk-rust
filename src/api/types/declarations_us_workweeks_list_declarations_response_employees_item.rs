pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsWorkweeksListDeclarationsResponseEmployeesItem {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(rename = "firstName")]
    #[serde(default)]
    pub first_name: String,
    #[serde(rename = "lastName")]
    #[serde(default)]
    pub last_name: String,
    #[serde(rename = "workweekStart")]
    #[serde(default)]
    pub workweek_start: String,
    #[serde(default)]
    pub exempt: bool,
    #[serde(default)]
    pub weeks: Vec<UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItem>,
}

impl UsWorkweeksListDeclarationsResponseEmployeesItem {
    pub fn builder() -> UsWorkweeksListDeclarationsResponseEmployeesItemBuilder {
        <UsWorkweeksListDeclarationsResponseEmployeesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsWorkweeksListDeclarationsResponseEmployeesItemBuilder {
    employee_id: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    workweek_start: Option<String>,
    exempt: Option<bool>,
    weeks: Option<Vec<UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItem>>,
}

impl UsWorkweeksListDeclarationsResponseEmployeesItemBuilder {
    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn workweek_start(mut self, value: impl Into<String>) -> Self {
        self.workweek_start = Some(value.into());
        self
    }

    pub fn exempt(mut self, value: bool) -> Self {
        self.exempt = Some(value);
        self
    }

    pub fn weeks(
        mut self,
        value: Vec<UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItem>,
    ) -> Self {
        self.weeks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsWorkweeksListDeclarationsResponseEmployeesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](UsWorkweeksListDeclarationsResponseEmployeesItemBuilder::employee_id)
    /// - [`first_name`](UsWorkweeksListDeclarationsResponseEmployeesItemBuilder::first_name)
    /// - [`last_name`](UsWorkweeksListDeclarationsResponseEmployeesItemBuilder::last_name)
    /// - [`workweek_start`](UsWorkweeksListDeclarationsResponseEmployeesItemBuilder::workweek_start)
    /// - [`exempt`](UsWorkweeksListDeclarationsResponseEmployeesItemBuilder::exempt)
    /// - [`weeks`](UsWorkweeksListDeclarationsResponseEmployeesItemBuilder::weeks)
    pub fn build(self) -> Result<UsWorkweeksListDeclarationsResponseEmployeesItem, BuildError> {
        Ok(UsWorkweeksListDeclarationsResponseEmployeesItem {
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            first_name: self
                .first_name
                .ok_or_else(|| BuildError::missing_field("first_name"))?,
            last_name: self
                .last_name
                .ok_or_else(|| BuildError::missing_field("last_name"))?,
            workweek_start: self
                .workweek_start
                .ok_or_else(|| BuildError::missing_field("workweek_start"))?,
            exempt: self
                .exempt
                .ok_or_else(|| BuildError::missing_field("exempt"))?,
            weeks: self
                .weeks
                .ok_or_else(|| BuildError::missing_field("weeks"))?,
        })
    }
}

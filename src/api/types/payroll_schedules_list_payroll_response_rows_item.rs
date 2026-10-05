pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SchedulesListPayrollResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "hoursPerWeek")]
    #[serde(default)]
    pub hours_per_week: String,
}

impl SchedulesListPayrollResponseRowsItem {
    pub fn builder() -> SchedulesListPayrollResponseRowsItemBuilder {
        <SchedulesListPayrollResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SchedulesListPayrollResponseRowsItemBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    hours_per_week: Option<String>,
}

impl SchedulesListPayrollResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn hours_per_week(mut self, value: impl Into<String>) -> Self {
        self.hours_per_week = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SchedulesListPayrollResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SchedulesListPayrollResponseRowsItemBuilder::id)
    /// - [`code`](SchedulesListPayrollResponseRowsItemBuilder::code)
    /// - [`name`](SchedulesListPayrollResponseRowsItemBuilder::name)
    /// - [`hours_per_week`](SchedulesListPayrollResponseRowsItemBuilder::hours_per_week)
    pub fn build(self) -> Result<SchedulesListPayrollResponseRowsItem, BuildError> {
        Ok(SchedulesListPayrollResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            hours_per_week: self
                .hours_per_week
                .ok_or_else(|| BuildError::missing_field("hours_per_week"))?,
        })
    }
}

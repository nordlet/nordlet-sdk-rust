pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollSchedulesListResponseRowsItem {
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

impl PostV1PayrollSchedulesListResponseRowsItem {
    pub fn builder() -> PostV1PayrollSchedulesListResponseRowsItemBuilder {
        <PostV1PayrollSchedulesListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollSchedulesListResponseRowsItemBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    hours_per_week: Option<String>,
}

impl PostV1PayrollSchedulesListResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1PayrollSchedulesListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PayrollSchedulesListResponseRowsItemBuilder::id)
    /// - [`code`](PostV1PayrollSchedulesListResponseRowsItemBuilder::code)
    /// - [`name`](PostV1PayrollSchedulesListResponseRowsItemBuilder::name)
    /// - [`hours_per_week`](PostV1PayrollSchedulesListResponseRowsItemBuilder::hours_per_week)
    pub fn build(self) -> Result<PostV1PayrollSchedulesListResponseRowsItem, BuildError> {
        Ok(PostV1PayrollSchedulesListResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            hours_per_week: self
                .hours_per_week
                .ok_or_else(|| BuildError::missing_field("hours_per_week"))?,
        })
    }
}

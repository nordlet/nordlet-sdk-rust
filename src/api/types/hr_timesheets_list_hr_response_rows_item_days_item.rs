pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimesheetsListHrResponseRowsItemDaysItem {
    #[serde(default)]
    pub day: i64,
    #[serde(default)]
    pub hours: String,
    pub r#type: TimesheetsListHrResponseRowsItemDaysItemType,
}

impl TimesheetsListHrResponseRowsItemDaysItem {
    pub fn builder() -> TimesheetsListHrResponseRowsItemDaysItemBuilder {
        <TimesheetsListHrResponseRowsItemDaysItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimesheetsListHrResponseRowsItemDaysItemBuilder {
    day: Option<i64>,
    hours: Option<String>,
    r#type: Option<TimesheetsListHrResponseRowsItemDaysItemType>,
}

impl TimesheetsListHrResponseRowsItemDaysItemBuilder {
    pub fn day(mut self, value: i64) -> Self {
        self.day = Some(value);
        self
    }

    pub fn hours(mut self, value: impl Into<String>) -> Self {
        self.hours = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: TimesheetsListHrResponseRowsItemDaysItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimesheetsListHrResponseRowsItemDaysItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`day`](TimesheetsListHrResponseRowsItemDaysItemBuilder::day)
    /// - [`hours`](TimesheetsListHrResponseRowsItemDaysItemBuilder::hours)
    /// - [`r#type`](TimesheetsListHrResponseRowsItemDaysItemBuilder::r#type)
    pub fn build(self) -> Result<TimesheetsListHrResponseRowsItemDaysItem, BuildError> {
        Ok(TimesheetsListHrResponseRowsItemDaysItem {
            day: self.day.ok_or_else(|| BuildError::missing_field("day"))?,
            hours: self
                .hours
                .ok_or_else(|| BuildError::missing_field("hours"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

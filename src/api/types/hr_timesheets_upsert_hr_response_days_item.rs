pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimesheetsUpsertHrResponseDaysItem {
    #[serde(default)]
    pub day: i64,
    #[serde(default)]
    pub hours: String,
    pub r#type: TimesheetsUpsertHrResponseDaysItemType,
}

impl TimesheetsUpsertHrResponseDaysItem {
    pub fn builder() -> TimesheetsUpsertHrResponseDaysItemBuilder {
        <TimesheetsUpsertHrResponseDaysItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimesheetsUpsertHrResponseDaysItemBuilder {
    day: Option<i64>,
    hours: Option<String>,
    r#type: Option<TimesheetsUpsertHrResponseDaysItemType>,
}

impl TimesheetsUpsertHrResponseDaysItemBuilder {
    pub fn day(mut self, value: i64) -> Self {
        self.day = Some(value);
        self
    }

    pub fn hours(mut self, value: impl Into<String>) -> Self {
        self.hours = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: TimesheetsUpsertHrResponseDaysItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimesheetsUpsertHrResponseDaysItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`day`](TimesheetsUpsertHrResponseDaysItemBuilder::day)
    /// - [`hours`](TimesheetsUpsertHrResponseDaysItemBuilder::hours)
    /// - [`r#type`](TimesheetsUpsertHrResponseDaysItemBuilder::r#type)
    pub fn build(self) -> Result<TimesheetsUpsertHrResponseDaysItem, BuildError> {
        Ok(TimesheetsUpsertHrResponseDaysItem {
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

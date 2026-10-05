pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimesheetsUpsertHrRequestDaysItem {
    #[serde(default)]
    pub day: i64,
    #[serde(default)]
    pub hours: String,
    pub r#type: TimesheetsUpsertHrRequestDaysItemType,
}

impl TimesheetsUpsertHrRequestDaysItem {
    pub fn builder() -> TimesheetsUpsertHrRequestDaysItemBuilder {
        <TimesheetsUpsertHrRequestDaysItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimesheetsUpsertHrRequestDaysItemBuilder {
    day: Option<i64>,
    hours: Option<String>,
    r#type: Option<TimesheetsUpsertHrRequestDaysItemType>,
}

impl TimesheetsUpsertHrRequestDaysItemBuilder {
    pub fn day(mut self, value: i64) -> Self {
        self.day = Some(value);
        self
    }

    pub fn hours(mut self, value: impl Into<String>) -> Self {
        self.hours = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: TimesheetsUpsertHrRequestDaysItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimesheetsUpsertHrRequestDaysItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`day`](TimesheetsUpsertHrRequestDaysItemBuilder::day)
    /// - [`hours`](TimesheetsUpsertHrRequestDaysItemBuilder::hours)
    /// - [`r#type`](TimesheetsUpsertHrRequestDaysItemBuilder::r#type)
    pub fn build(self) -> Result<TimesheetsUpsertHrRequestDaysItem, BuildError> {
        Ok(TimesheetsUpsertHrRequestDaysItem {
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

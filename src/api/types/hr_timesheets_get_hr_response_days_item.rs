pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimesheetsGetHrResponseDaysItem {
    #[serde(default)]
    pub day: i64,
    #[serde(default)]
    pub hours: String,
    pub r#type: TimesheetsGetHrResponseDaysItemType,
}

impl TimesheetsGetHrResponseDaysItem {
    pub fn builder() -> TimesheetsGetHrResponseDaysItemBuilder {
        <TimesheetsGetHrResponseDaysItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimesheetsGetHrResponseDaysItemBuilder {
    day: Option<i64>,
    hours: Option<String>,
    r#type: Option<TimesheetsGetHrResponseDaysItemType>,
}

impl TimesheetsGetHrResponseDaysItemBuilder {
    pub fn day(mut self, value: i64) -> Self {
        self.day = Some(value);
        self
    }

    pub fn hours(mut self, value: impl Into<String>) -> Self {
        self.hours = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: TimesheetsGetHrResponseDaysItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimesheetsGetHrResponseDaysItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`day`](TimesheetsGetHrResponseDaysItemBuilder::day)
    /// - [`hours`](TimesheetsGetHrResponseDaysItemBuilder::hours)
    /// - [`r#type`](TimesheetsGetHrResponseDaysItemBuilder::r#type)
    pub fn build(self) -> Result<TimesheetsGetHrResponseDaysItem, BuildError> {
        Ok(TimesheetsGetHrResponseDaysItem {
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

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1HrTimesheetsUpsertResponseDaysItem {
    #[serde(default)]
    pub day: i64,
    #[serde(default)]
    pub hours: String,
    pub r#type: PostV1HrTimesheetsUpsertResponseDaysItemType,
}

impl PostV1HrTimesheetsUpsertResponseDaysItem {
    pub fn builder() -> PostV1HrTimesheetsUpsertResponseDaysItemBuilder {
        <PostV1HrTimesheetsUpsertResponseDaysItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrTimesheetsUpsertResponseDaysItemBuilder {
    day: Option<i64>,
    hours: Option<String>,
    r#type: Option<PostV1HrTimesheetsUpsertResponseDaysItemType>,
}

impl PostV1HrTimesheetsUpsertResponseDaysItemBuilder {
    pub fn day(mut self, value: i64) -> Self {
        self.day = Some(value);
        self
    }

    pub fn hours(mut self, value: impl Into<String>) -> Self {
        self.hours = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1HrTimesheetsUpsertResponseDaysItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrTimesheetsUpsertResponseDaysItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`day`](PostV1HrTimesheetsUpsertResponseDaysItemBuilder::day)
    /// - [`hours`](PostV1HrTimesheetsUpsertResponseDaysItemBuilder::hours)
    /// - [`r#type`](PostV1HrTimesheetsUpsertResponseDaysItemBuilder::r#type)
    pub fn build(self) -> Result<PostV1HrTimesheetsUpsertResponseDaysItem, BuildError> {
        Ok(PostV1HrTimesheetsUpsertResponseDaysItem {
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

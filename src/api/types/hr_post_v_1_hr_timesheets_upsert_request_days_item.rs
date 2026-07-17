pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1HrTimesheetsUpsertRequestDaysItem {
    #[serde(default)]
    pub day: i64,
    #[serde(default)]
    pub hours: String,
    pub r#type: PostV1HrTimesheetsUpsertRequestDaysItemType,
}

impl PostV1HrTimesheetsUpsertRequestDaysItem {
    pub fn builder() -> PostV1HrTimesheetsUpsertRequestDaysItemBuilder {
        <PostV1HrTimesheetsUpsertRequestDaysItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrTimesheetsUpsertRequestDaysItemBuilder {
    day: Option<i64>,
    hours: Option<String>,
    r#type: Option<PostV1HrTimesheetsUpsertRequestDaysItemType>,
}

impl PostV1HrTimesheetsUpsertRequestDaysItemBuilder {
    pub fn day(mut self, value: i64) -> Self {
        self.day = Some(value);
        self
    }

    pub fn hours(mut self, value: impl Into<String>) -> Self {
        self.hours = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1HrTimesheetsUpsertRequestDaysItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrTimesheetsUpsertRequestDaysItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`day`](PostV1HrTimesheetsUpsertRequestDaysItemBuilder::day)
    /// - [`hours`](PostV1HrTimesheetsUpsertRequestDaysItemBuilder::hours)
    /// - [`r#type`](PostV1HrTimesheetsUpsertRequestDaysItemBuilder::r#type)
    pub fn build(self) -> Result<PostV1HrTimesheetsUpsertRequestDaysItem, BuildError> {
        Ok(PostV1HrTimesheetsUpsertRequestDaysItem {
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

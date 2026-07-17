pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollSchedulesCreateRequest {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "hoursPerWeek")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_per_week: Option<String>,
}

impl PostV1PayrollSchedulesCreateRequest {
    pub fn builder() -> PostV1PayrollSchedulesCreateRequestBuilder {
        <PostV1PayrollSchedulesCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollSchedulesCreateRequestBuilder {
    code: Option<String>,
    name: Option<String>,
    hours_per_week: Option<String>,
}

impl PostV1PayrollSchedulesCreateRequestBuilder {
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

    /// Consumes the builder and constructs a [`PostV1PayrollSchedulesCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1PayrollSchedulesCreateRequestBuilder::code)
    /// - [`name`](PostV1PayrollSchedulesCreateRequestBuilder::name)
    pub fn build(self) -> Result<PostV1PayrollSchedulesCreateRequest, BuildError> {
        Ok(PostV1PayrollSchedulesCreateRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            hours_per_week: self.hours_per_week,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollRunsCreateRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<PostV1PayrollRunsCreateRequestLinesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1PayrollRunsCreateRequest {
    pub fn builder() -> PostV1PayrollRunsCreateRequestBuilder {
        <PostV1PayrollRunsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollRunsCreateRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
    lines: Option<Vec<PostV1PayrollRunsCreateRequestLinesItem>>,
    notes: Option<String>,
}

impl PostV1PayrollRunsCreateRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn lines(mut self, value: Vec<PostV1PayrollRunsCreateRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PayrollRunsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1PayrollRunsCreateRequestBuilder::year)
    /// - [`month`](PostV1PayrollRunsCreateRequestBuilder::month)
    pub fn build(self) -> Result<PostV1PayrollRunsCreateRequest, BuildError> {
        Ok(PostV1PayrollRunsCreateRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            lines: self.lines,
            notes: self.notes,
        })
    }
}

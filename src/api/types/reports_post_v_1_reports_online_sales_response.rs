pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsOnlineSalesResponse {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(default)]
    pub rows: Vec<PostV1ReportsOnlineSalesResponseRowsItem>,
}

impl PostV1ReportsOnlineSalesResponse {
    pub fn builder() -> PostV1ReportsOnlineSalesResponseBuilder {
        <PostV1ReportsOnlineSalesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsOnlineSalesResponseBuilder {
    from_date: Option<String>,
    to_date: Option<String>,
    rows: Option<Vec<PostV1ReportsOnlineSalesResponseRowsItem>>,
}

impl PostV1ReportsOnlineSalesResponseBuilder {
    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<PostV1ReportsOnlineSalesResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsOnlineSalesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](PostV1ReportsOnlineSalesResponseBuilder::from_date)
    /// - [`to_date`](PostV1ReportsOnlineSalesResponseBuilder::to_date)
    /// - [`rows`](PostV1ReportsOnlineSalesResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1ReportsOnlineSalesResponse, BuildError> {
        Ok(PostV1ReportsOnlineSalesResponse {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}

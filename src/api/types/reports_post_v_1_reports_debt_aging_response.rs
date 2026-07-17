pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsDebtAgingResponse {
    #[serde(rename = "asOf")]
    #[serde(default)]
    pub as_of: String,
    #[serde(default)]
    pub side: String,
    #[serde(default)]
    pub rows: Vec<PostV1ReportsDebtAgingResponseRowsItem>,
}

impl PostV1ReportsDebtAgingResponse {
    pub fn builder() -> PostV1ReportsDebtAgingResponseBuilder {
        <PostV1ReportsDebtAgingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsDebtAgingResponseBuilder {
    as_of: Option<String>,
    side: Option<String>,
    rows: Option<Vec<PostV1ReportsDebtAgingResponseRowsItem>>,
}

impl PostV1ReportsDebtAgingResponseBuilder {
    pub fn as_of(mut self, value: impl Into<String>) -> Self {
        self.as_of = Some(value.into());
        self
    }

    pub fn side(mut self, value: impl Into<String>) -> Self {
        self.side = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<PostV1ReportsDebtAgingResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsDebtAgingResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`as_of`](PostV1ReportsDebtAgingResponseBuilder::as_of)
    /// - [`side`](PostV1ReportsDebtAgingResponseBuilder::side)
    /// - [`rows`](PostV1ReportsDebtAgingResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1ReportsDebtAgingResponse, BuildError> {
        Ok(PostV1ReportsDebtAgingResponse {
            as_of: self
                .as_of
                .ok_or_else(|| BuildError::missing_field("as_of"))?,
            side: self.side.ok_or_else(|| BuildError::missing_field("side"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}

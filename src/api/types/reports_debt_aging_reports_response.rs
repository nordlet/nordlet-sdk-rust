pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DebtAgingReportsResponse {
    #[serde(rename = "asOf")]
    #[serde(default)]
    pub as_of: String,
    #[serde(default)]
    pub side: String,
    #[serde(default)]
    pub rows: Vec<DebtAgingReportsResponseRowsItem>,
}

impl DebtAgingReportsResponse {
    pub fn builder() -> DebtAgingReportsResponseBuilder {
        <DebtAgingReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DebtAgingReportsResponseBuilder {
    as_of: Option<String>,
    side: Option<String>,
    rows: Option<Vec<DebtAgingReportsResponseRowsItem>>,
}

impl DebtAgingReportsResponseBuilder {
    pub fn as_of(mut self, value: impl Into<String>) -> Self {
        self.as_of = Some(value.into());
        self
    }

    pub fn side(mut self, value: impl Into<String>) -> Self {
        self.side = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<DebtAgingReportsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DebtAgingReportsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`as_of`](DebtAgingReportsResponseBuilder::as_of)
    /// - [`side`](DebtAgingReportsResponseBuilder::side)
    /// - [`rows`](DebtAgingReportsResponseBuilder::rows)
    pub fn build(self) -> Result<DebtAgingReportsResponse, BuildError> {
        Ok(DebtAgingReportsResponse {
            as_of: self
                .as_of
                .ok_or_else(|| BuildError::missing_field("as_of"))?,
            side: self.side.ok_or_else(|| BuildError::missing_field("side"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}

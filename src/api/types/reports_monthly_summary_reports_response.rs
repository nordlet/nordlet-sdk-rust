pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MonthlySummaryReportsResponse {
    #[serde(default)]
    pub rows: Vec<MonthlySummaryReportsResponseRowsItem>,
}

impl MonthlySummaryReportsResponse {
    pub fn builder() -> MonthlySummaryReportsResponseBuilder {
        <MonthlySummaryReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MonthlySummaryReportsResponseBuilder {
    rows: Option<Vec<MonthlySummaryReportsResponseRowsItem>>,
}

impl MonthlySummaryReportsResponseBuilder {
    pub fn rows(mut self, value: Vec<MonthlySummaryReportsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MonthlySummaryReportsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](MonthlySummaryReportsResponseBuilder::rows)
    pub fn build(self) -> Result<MonthlySummaryReportsResponse, BuildError> {
        Ok(MonthlySummaryReportsResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}

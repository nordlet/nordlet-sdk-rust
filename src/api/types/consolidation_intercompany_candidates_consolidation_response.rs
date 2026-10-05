pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IntercompanyCandidatesConsolidationResponse {
    #[serde(default)]
    pub rows: Vec<IntercompanyCandidatesConsolidationResponseRowsItem>,
}

impl IntercompanyCandidatesConsolidationResponse {
    pub fn builder() -> IntercompanyCandidatesConsolidationResponseBuilder {
        <IntercompanyCandidatesConsolidationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IntercompanyCandidatesConsolidationResponseBuilder {
    rows: Option<Vec<IntercompanyCandidatesConsolidationResponseRowsItem>>,
}

impl IntercompanyCandidatesConsolidationResponseBuilder {
    pub fn rows(mut self, value: Vec<IntercompanyCandidatesConsolidationResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IntercompanyCandidatesConsolidationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](IntercompanyCandidatesConsolidationResponseBuilder::rows)
    pub fn build(self) -> Result<IntercompanyCandidatesConsolidationResponse, BuildError> {
        Ok(IntercompanyCandidatesConsolidationResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}

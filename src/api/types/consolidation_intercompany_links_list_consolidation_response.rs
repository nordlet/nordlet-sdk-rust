pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IntercompanyLinksListConsolidationResponse {
    #[serde(default)]
    pub rows: Vec<IntercompanyLinksListConsolidationResponseRowsItem>,
}

impl IntercompanyLinksListConsolidationResponse {
    pub fn builder() -> IntercompanyLinksListConsolidationResponseBuilder {
        <IntercompanyLinksListConsolidationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IntercompanyLinksListConsolidationResponseBuilder {
    rows: Option<Vec<IntercompanyLinksListConsolidationResponseRowsItem>>,
}

impl IntercompanyLinksListConsolidationResponseBuilder {
    pub fn rows(mut self, value: Vec<IntercompanyLinksListConsolidationResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IntercompanyLinksListConsolidationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](IntercompanyLinksListConsolidationResponseBuilder::rows)
    pub fn build(self) -> Result<IntercompanyLinksListConsolidationResponse, BuildError> {
        Ok(IntercompanyLinksListConsolidationResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}

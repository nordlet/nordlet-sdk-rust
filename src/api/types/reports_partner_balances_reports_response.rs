pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PartnerBalancesReportsResponse {
    #[serde(default)]
    pub rows: Vec<PartnerBalancesReportsResponseRowsItem>,
}

impl PartnerBalancesReportsResponse {
    pub fn builder() -> PartnerBalancesReportsResponseBuilder {
        <PartnerBalancesReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PartnerBalancesReportsResponseBuilder {
    rows: Option<Vec<PartnerBalancesReportsResponseRowsItem>>,
}

impl PartnerBalancesReportsResponseBuilder {
    pub fn rows(mut self, value: Vec<PartnerBalancesReportsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PartnerBalancesReportsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PartnerBalancesReportsResponseBuilder::rows)
    pub fn build(self) -> Result<PartnerBalancesReportsResponse, BuildError> {
        Ok(PartnerBalancesReportsResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}

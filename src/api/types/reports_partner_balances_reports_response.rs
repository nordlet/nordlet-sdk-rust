pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PartnerBalancesReportsResponse {
    #[serde(default)]
    pub rows: Vec<PartnerBalancesReportsResponseRowsItem>,
    #[serde(default)]
    pub totals: PartnerBalancesReportsResponseTotals,
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
    totals: Option<PartnerBalancesReportsResponseTotals>,
}

impl PartnerBalancesReportsResponseBuilder {
    pub fn rows(mut self, value: Vec<PartnerBalancesReportsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn totals(mut self, value: PartnerBalancesReportsResponseTotals) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PartnerBalancesReportsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PartnerBalancesReportsResponseBuilder::rows)
    /// - [`totals`](PartnerBalancesReportsResponseBuilder::totals)
    pub fn build(self) -> Result<PartnerBalancesReportsResponse, BuildError> {
        Ok(PartnerBalancesReportsResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            totals: self
                .totals
                .ok_or_else(|| BuildError::missing_field("totals"))?,
        })
    }
}

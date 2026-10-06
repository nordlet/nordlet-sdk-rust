pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PartnerBalancesReportsResponseTotals {
    #[serde(default)]
    pub receivable: String,
    #[serde(default)]
    pub payable: String,
}

impl PartnerBalancesReportsResponseTotals {
    pub fn builder() -> PartnerBalancesReportsResponseTotalsBuilder {
        <PartnerBalancesReportsResponseTotalsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PartnerBalancesReportsResponseTotalsBuilder {
    receivable: Option<String>,
    payable: Option<String>,
}

impl PartnerBalancesReportsResponseTotalsBuilder {
    pub fn receivable(mut self, value: impl Into<String>) -> Self {
        self.receivable = Some(value.into());
        self
    }

    pub fn payable(mut self, value: impl Into<String>) -> Self {
        self.payable = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PartnerBalancesReportsResponseTotals`].
    /// This method will fail if any of the following fields are not set:
    /// - [`receivable`](PartnerBalancesReportsResponseTotalsBuilder::receivable)
    /// - [`payable`](PartnerBalancesReportsResponseTotalsBuilder::payable)
    pub fn build(self) -> Result<PartnerBalancesReportsResponseTotals, BuildError> {
        Ok(PartnerBalancesReportsResponseTotals {
            receivable: self
                .receivable
                .ok_or_else(|| BuildError::missing_field("receivable"))?,
            payable: self
                .payable
                .ok_or_else(|| BuildError::missing_field("payable"))?,
        })
    }
}

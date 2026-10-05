pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuPurchasesReportsResponseTotals {
    #[serde(default)]
    pub net: String,
    #[serde(default)]
    pub vat: String,
}

impl EuPurchasesReportsResponseTotals {
    pub fn builder() -> EuPurchasesReportsResponseTotalsBuilder {
        <EuPurchasesReportsResponseTotalsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuPurchasesReportsResponseTotalsBuilder {
    net: Option<String>,
    vat: Option<String>,
}

impl EuPurchasesReportsResponseTotalsBuilder {
    pub fn net(mut self, value: impl Into<String>) -> Self {
        self.net = Some(value.into());
        self
    }

    pub fn vat(mut self, value: impl Into<String>) -> Self {
        self.vat = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EuPurchasesReportsResponseTotals`].
    /// This method will fail if any of the following fields are not set:
    /// - [`net`](EuPurchasesReportsResponseTotalsBuilder::net)
    /// - [`vat`](EuPurchasesReportsResponseTotalsBuilder::vat)
    pub fn build(self) -> Result<EuPurchasesReportsResponseTotals, BuildError> {
        Ok(EuPurchasesReportsResponseTotals {
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
            vat: self.vat.ok_or_else(|| BuildError::missing_field("vat"))?,
        })
    }
}

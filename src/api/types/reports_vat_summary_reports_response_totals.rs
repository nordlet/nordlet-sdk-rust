pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatSummaryReportsResponseTotals {
    #[serde(default)]
    pub net: String,
    #[serde(default)]
    pub vat: String,
    #[serde(default)]
    pub gross: String,
}

impl VatSummaryReportsResponseTotals {
    pub fn builder() -> VatSummaryReportsResponseTotalsBuilder {
        <VatSummaryReportsResponseTotalsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatSummaryReportsResponseTotalsBuilder {
    net: Option<String>,
    vat: Option<String>,
    gross: Option<String>,
}

impl VatSummaryReportsResponseTotalsBuilder {
    pub fn net(mut self, value: impl Into<String>) -> Self {
        self.net = Some(value.into());
        self
    }

    pub fn vat(mut self, value: impl Into<String>) -> Self {
        self.vat = Some(value.into());
        self
    }

    pub fn gross(mut self, value: impl Into<String>) -> Self {
        self.gross = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VatSummaryReportsResponseTotals`].
    /// This method will fail if any of the following fields are not set:
    /// - [`net`](VatSummaryReportsResponseTotalsBuilder::net)
    /// - [`vat`](VatSummaryReportsResponseTotalsBuilder::vat)
    /// - [`gross`](VatSummaryReportsResponseTotalsBuilder::gross)
    pub fn build(self) -> Result<VatSummaryReportsResponseTotals, BuildError> {
        Ok(VatSummaryReportsResponseTotals {
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
            vat: self.vat.ok_or_else(|| BuildError::missing_field("vat"))?,
            gross: self
                .gross
                .ok_or_else(|| BuildError::missing_field("gross"))?,
        })
    }
}

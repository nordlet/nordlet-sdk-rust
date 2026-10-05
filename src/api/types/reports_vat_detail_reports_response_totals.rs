pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatDetailReportsResponseTotals {
    #[serde(default)]
    pub net: String,
    #[serde(default)]
    pub vat: String,
    #[serde(default)]
    pub gross: String,
}

impl VatDetailReportsResponseTotals {
    pub fn builder() -> VatDetailReportsResponseTotalsBuilder {
        <VatDetailReportsResponseTotalsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatDetailReportsResponseTotalsBuilder {
    net: Option<String>,
    vat: Option<String>,
    gross: Option<String>,
}

impl VatDetailReportsResponseTotalsBuilder {
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

    /// Consumes the builder and constructs a [`VatDetailReportsResponseTotals`].
    /// This method will fail if any of the following fields are not set:
    /// - [`net`](VatDetailReportsResponseTotalsBuilder::net)
    /// - [`vat`](VatDetailReportsResponseTotalsBuilder::vat)
    /// - [`gross`](VatDetailReportsResponseTotalsBuilder::gross)
    pub fn build(self) -> Result<VatDetailReportsResponseTotals, BuildError> {
        Ok(VatDetailReportsResponseTotals {
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
            vat: self.vat.ok_or_else(|| BuildError::missing_field("vat"))?,
            gross: self
                .gross
                .ok_or_else(|| BuildError::missing_field("gross"))?,
        })
    }
}

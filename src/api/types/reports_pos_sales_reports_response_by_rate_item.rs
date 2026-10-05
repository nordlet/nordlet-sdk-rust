pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PosSalesReportsResponseByRateItem {
    #[serde(rename = "vatRatePercent")]
    #[serde(default)]
    pub vat_rate_percent: String,
    #[serde(default)]
    pub net: String,
    #[serde(default)]
    pub vat: String,
}

impl PosSalesReportsResponseByRateItem {
    pub fn builder() -> PosSalesReportsResponseByRateItemBuilder {
        <PosSalesReportsResponseByRateItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PosSalesReportsResponseByRateItemBuilder {
    vat_rate_percent: Option<String>,
    net: Option<String>,
    vat: Option<String>,
}

impl PosSalesReportsResponseByRateItemBuilder {
    pub fn vat_rate_percent(mut self, value: impl Into<String>) -> Self {
        self.vat_rate_percent = Some(value.into());
        self
    }

    pub fn net(mut self, value: impl Into<String>) -> Self {
        self.net = Some(value.into());
        self
    }

    pub fn vat(mut self, value: impl Into<String>) -> Self {
        self.vat = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PosSalesReportsResponseByRateItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`vat_rate_percent`](PosSalesReportsResponseByRateItemBuilder::vat_rate_percent)
    /// - [`net`](PosSalesReportsResponseByRateItemBuilder::net)
    /// - [`vat`](PosSalesReportsResponseByRateItemBuilder::vat)
    pub fn build(self) -> Result<PosSalesReportsResponseByRateItem, BuildError> {
        Ok(PosSalesReportsResponseByRateItem {
            vat_rate_percent: self
                .vat_rate_percent
                .ok_or_else(|| BuildError::missing_field("vat_rate_percent"))?,
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
            vat: self.vat.ok_or_else(|| BuildError::missing_field("vat"))?,
        })
    }
}

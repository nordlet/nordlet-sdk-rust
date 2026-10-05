pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportsGetPosResponseVatLinesItem {
    #[serde(rename = "vatRatePercent")]
    #[serde(default)]
    pub vat_rate_percent: String,
    #[serde(rename = "netAmount")]
    #[serde(default)]
    pub net_amount: String,
    #[serde(rename = "vatAmount")]
    #[serde(default)]
    pub vat_amount: String,
}

impl ReportsGetPosResponseVatLinesItem {
    pub fn builder() -> ReportsGetPosResponseVatLinesItemBuilder {
        <ReportsGetPosResponseVatLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportsGetPosResponseVatLinesItemBuilder {
    vat_rate_percent: Option<String>,
    net_amount: Option<String>,
    vat_amount: Option<String>,
}

impl ReportsGetPosResponseVatLinesItemBuilder {
    pub fn vat_rate_percent(mut self, value: impl Into<String>) -> Self {
        self.vat_rate_percent = Some(value.into());
        self
    }

    pub fn net_amount(mut self, value: impl Into<String>) -> Self {
        self.net_amount = Some(value.into());
        self
    }

    pub fn vat_amount(mut self, value: impl Into<String>) -> Self {
        self.vat_amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReportsGetPosResponseVatLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`vat_rate_percent`](ReportsGetPosResponseVatLinesItemBuilder::vat_rate_percent)
    /// - [`net_amount`](ReportsGetPosResponseVatLinesItemBuilder::net_amount)
    /// - [`vat_amount`](ReportsGetPosResponseVatLinesItemBuilder::vat_amount)
    pub fn build(self) -> Result<ReportsGetPosResponseVatLinesItem, BuildError> {
        Ok(ReportsGetPosResponseVatLinesItem {
            vat_rate_percent: self
                .vat_rate_percent
                .ok_or_else(|| BuildError::missing_field("vat_rate_percent"))?,
            net_amount: self
                .net_amount
                .ok_or_else(|| BuildError::missing_field("net_amount"))?,
            vat_amount: self
                .vat_amount
                .ok_or_else(|| BuildError::missing_field("vat_amount"))?,
        })
    }
}

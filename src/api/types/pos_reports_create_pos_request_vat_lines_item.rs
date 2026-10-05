pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportsCreatePosRequestVatLinesItem {
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

impl ReportsCreatePosRequestVatLinesItem {
    pub fn builder() -> ReportsCreatePosRequestVatLinesItemBuilder {
        <ReportsCreatePosRequestVatLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportsCreatePosRequestVatLinesItemBuilder {
    vat_rate_percent: Option<String>,
    net_amount: Option<String>,
    vat_amount: Option<String>,
}

impl ReportsCreatePosRequestVatLinesItemBuilder {
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

    /// Consumes the builder and constructs a [`ReportsCreatePosRequestVatLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`vat_rate_percent`](ReportsCreatePosRequestVatLinesItemBuilder::vat_rate_percent)
    /// - [`net_amount`](ReportsCreatePosRequestVatLinesItemBuilder::net_amount)
    /// - [`vat_amount`](ReportsCreatePosRequestVatLinesItemBuilder::vat_amount)
    pub fn build(self) -> Result<ReportsCreatePosRequestVatLinesItem, BuildError> {
        Ok(ReportsCreatePosRequestVatLinesItem {
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

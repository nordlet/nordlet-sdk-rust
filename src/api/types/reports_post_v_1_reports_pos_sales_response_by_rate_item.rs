pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsPosSalesResponseByRateItem {
    #[serde(rename = "vatRatePercent")]
    #[serde(default)]
    pub vat_rate_percent: String,
    #[serde(default)]
    pub net: String,
    #[serde(default)]
    pub vat: String,
}

impl PostV1ReportsPosSalesResponseByRateItem {
    pub fn builder() -> PostV1ReportsPosSalesResponseByRateItemBuilder {
        <PostV1ReportsPosSalesResponseByRateItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsPosSalesResponseByRateItemBuilder {
    vat_rate_percent: Option<String>,
    net: Option<String>,
    vat: Option<String>,
}

impl PostV1ReportsPosSalesResponseByRateItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1ReportsPosSalesResponseByRateItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`vat_rate_percent`](PostV1ReportsPosSalesResponseByRateItemBuilder::vat_rate_percent)
    /// - [`net`](PostV1ReportsPosSalesResponseByRateItemBuilder::net)
    /// - [`vat`](PostV1ReportsPosSalesResponseByRateItemBuilder::vat)
    pub fn build(self) -> Result<PostV1ReportsPosSalesResponseByRateItem, BuildError> {
        Ok(PostV1ReportsPosSalesResponseByRateItem {
            vat_rate_percent: self
                .vat_rate_percent
                .ok_or_else(|| BuildError::missing_field("vat_rate_percent"))?,
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
            vat: self.vat.ok_or_else(|| BuildError::missing_field("vat"))?,
        })
    }
}

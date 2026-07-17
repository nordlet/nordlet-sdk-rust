pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PosReportsCreateRequestVatLinesItem {
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

impl PostV1PosReportsCreateRequestVatLinesItem {
    pub fn builder() -> PostV1PosReportsCreateRequestVatLinesItemBuilder {
        <PostV1PosReportsCreateRequestVatLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PosReportsCreateRequestVatLinesItemBuilder {
    vat_rate_percent: Option<String>,
    net_amount: Option<String>,
    vat_amount: Option<String>,
}

impl PostV1PosReportsCreateRequestVatLinesItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1PosReportsCreateRequestVatLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`vat_rate_percent`](PostV1PosReportsCreateRequestVatLinesItemBuilder::vat_rate_percent)
    /// - [`net_amount`](PostV1PosReportsCreateRequestVatLinesItemBuilder::net_amount)
    /// - [`vat_amount`](PostV1PosReportsCreateRequestVatLinesItemBuilder::vat_amount)
    pub fn build(self) -> Result<PostV1PosReportsCreateRequestVatLinesItem, BuildError> {
        Ok(PostV1PosReportsCreateRequestVatLinesItem {
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

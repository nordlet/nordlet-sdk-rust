pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PosReportsCreateResponseVatLinesItem {
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

impl PostV1PosReportsCreateResponseVatLinesItem {
    pub fn builder() -> PostV1PosReportsCreateResponseVatLinesItemBuilder {
        <PostV1PosReportsCreateResponseVatLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PosReportsCreateResponseVatLinesItemBuilder {
    vat_rate_percent: Option<String>,
    net_amount: Option<String>,
    vat_amount: Option<String>,
}

impl PostV1PosReportsCreateResponseVatLinesItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1PosReportsCreateResponseVatLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`vat_rate_percent`](PostV1PosReportsCreateResponseVatLinesItemBuilder::vat_rate_percent)
    /// - [`net_amount`](PostV1PosReportsCreateResponseVatLinesItemBuilder::net_amount)
    /// - [`vat_amount`](PostV1PosReportsCreateResponseVatLinesItemBuilder::vat_amount)
    pub fn build(self) -> Result<PostV1PosReportsCreateResponseVatLinesItem, BuildError> {
        Ok(PostV1PosReportsCreateResponseVatLinesItem {
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

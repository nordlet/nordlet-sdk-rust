pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CaptureDocumentsUploadResponseExtractionLinesItem {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub quantity: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "unitPriceExclVat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price_excl_vat: Option<String>,
    #[serde(rename = "vatRatePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_rate_percent: Option<String>,
    #[serde(rename = "lineNet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_net: Option<String>,
    #[serde(rename = "lineVat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_vat: Option<String>,
    #[serde(rename = "lineGross")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_gross: Option<String>,
}

impl PostV1CaptureDocumentsUploadResponseExtractionLinesItem {
    pub fn builder() -> PostV1CaptureDocumentsUploadResponseExtractionLinesItemBuilder {
        <PostV1CaptureDocumentsUploadResponseExtractionLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CaptureDocumentsUploadResponseExtractionLinesItemBuilder {
    description: Option<String>,
    quantity: Option<String>,
    unit: Option<String>,
    unit_price_excl_vat: Option<String>,
    vat_rate_percent: Option<String>,
    line_net: Option<String>,
    line_vat: Option<String>,
    line_gross: Option<String>,
}

impl PostV1CaptureDocumentsUploadResponseExtractionLinesItemBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    pub fn unit_price_excl_vat(mut self, value: impl Into<String>) -> Self {
        self.unit_price_excl_vat = Some(value.into());
        self
    }

    pub fn vat_rate_percent(mut self, value: impl Into<String>) -> Self {
        self.vat_rate_percent = Some(value.into());
        self
    }

    pub fn line_net(mut self, value: impl Into<String>) -> Self {
        self.line_net = Some(value.into());
        self
    }

    pub fn line_vat(mut self, value: impl Into<String>) -> Self {
        self.line_vat = Some(value.into());
        self
    }

    pub fn line_gross(mut self, value: impl Into<String>) -> Self {
        self.line_gross = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CaptureDocumentsUploadResponseExtractionLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](PostV1CaptureDocumentsUploadResponseExtractionLinesItemBuilder::description)
    /// - [`quantity`](PostV1CaptureDocumentsUploadResponseExtractionLinesItemBuilder::quantity)
    pub fn build(
        self,
    ) -> Result<PostV1CaptureDocumentsUploadResponseExtractionLinesItem, BuildError> {
        Ok(PostV1CaptureDocumentsUploadResponseExtractionLinesItem {
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            unit: self.unit,
            unit_price_excl_vat: self.unit_price_excl_vat,
            vat_rate_percent: self.vat_rate_percent,
            line_net: self.line_net,
            line_vat: self.line_vat,
            line_gross: self.line_gross,
        })
    }
}

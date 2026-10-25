pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesGetPublicResponseLinesItem {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub unit: String,
    #[serde(default)]
    pub quantity: String,
    #[serde(rename = "unitPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<String>,
    #[serde(rename = "vatRatePercent")]
    #[serde(default)]
    pub vat_rate_percent: String,
    #[serde(rename = "lineNet")]
    #[serde(default)]
    pub line_net: String,
}

impl InvoicesGetPublicResponseLinesItem {
    pub fn builder() -> InvoicesGetPublicResponseLinesItemBuilder {
        <InvoicesGetPublicResponseLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesGetPublicResponseLinesItemBuilder {
    description: Option<String>,
    unit: Option<String>,
    quantity: Option<String>,
    unit_price: Option<String>,
    vat_rate_percent: Option<String>,
    line_net: Option<String>,
}

impl InvoicesGetPublicResponseLinesItemBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    pub fn unit_price(mut self, value: impl Into<String>) -> Self {
        self.unit_price = Some(value.into());
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

    /// Consumes the builder and constructs a [`InvoicesGetPublicResponseLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](InvoicesGetPublicResponseLinesItemBuilder::description)
    /// - [`unit`](InvoicesGetPublicResponseLinesItemBuilder::unit)
    /// - [`quantity`](InvoicesGetPublicResponseLinesItemBuilder::quantity)
    /// - [`vat_rate_percent`](InvoicesGetPublicResponseLinesItemBuilder::vat_rate_percent)
    /// - [`line_net`](InvoicesGetPublicResponseLinesItemBuilder::line_net)
    pub fn build(self) -> Result<InvoicesGetPublicResponseLinesItem, BuildError> {
        Ok(InvoicesGetPublicResponseLinesItem {
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            unit_price: self.unit_price,
            vat_rate_percent: self
                .vat_rate_percent
                .ok_or_else(|| BuildError::missing_field("vat_rate_percent"))?,
            line_net: self
                .line_net
                .ok_or_else(|| BuildError::missing_field("line_net"))?,
        })
    }
}

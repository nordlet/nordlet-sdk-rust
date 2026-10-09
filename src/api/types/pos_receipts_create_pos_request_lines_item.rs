pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceiptsCreatePosRequestLinesItem {
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub quantity: String,
    #[serde(rename = "unitPriceInclVat")]
    #[serde(default)]
    pub unit_price_incl_vat: String,
    #[serde(rename = "vatRatePercent")]
    #[serde(default)]
    pub vat_rate_percent: String,
}

impl ReceiptsCreatePosRequestLinesItem {
    pub fn builder() -> ReceiptsCreatePosRequestLinesItemBuilder {
        <ReceiptsCreatePosRequestLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceiptsCreatePosRequestLinesItemBuilder {
    item_id: Option<String>,
    description: Option<String>,
    quantity: Option<String>,
    unit_price_incl_vat: Option<String>,
    vat_rate_percent: Option<String>,
}

impl ReceiptsCreatePosRequestLinesItemBuilder {
    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    pub fn unit_price_incl_vat(mut self, value: impl Into<String>) -> Self {
        self.unit_price_incl_vat = Some(value.into());
        self
    }

    pub fn vat_rate_percent(mut self, value: impl Into<String>) -> Self {
        self.vat_rate_percent = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReceiptsCreatePosRequestLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`quantity`](ReceiptsCreatePosRequestLinesItemBuilder::quantity)
    /// - [`unit_price_incl_vat`](ReceiptsCreatePosRequestLinesItemBuilder::unit_price_incl_vat)
    /// - [`vat_rate_percent`](ReceiptsCreatePosRequestLinesItemBuilder::vat_rate_percent)
    pub fn build(self) -> Result<ReceiptsCreatePosRequestLinesItem, BuildError> {
        Ok(ReceiptsCreatePosRequestLinesItem {
            item_id: self.item_id,
            description: self.description,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            unit_price_incl_vat: self
                .unit_price_incl_vat
                .ok_or_else(|| BuildError::missing_field("unit_price_incl_vat"))?,
            vat_rate_percent: self
                .vat_rate_percent
                .ok_or_else(|| BuildError::missing_field("vat_rate_percent"))?,
        })
    }
}

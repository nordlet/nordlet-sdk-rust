pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceiptsGetPosResponseLinesItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub quantity: String,
    #[serde(rename = "unitPriceInclVat")]
    #[serde(default)]
    pub unit_price_incl_vat: String,
    #[serde(rename = "vatRatePercent")]
    #[serde(default)]
    pub vat_rate_percent: String,
    #[serde(rename = "netAmount")]
    #[serde(default)]
    pub net_amount: String,
    #[serde(rename = "vatAmount")]
    #[serde(default)]
    pub vat_amount: String,
    #[serde(rename = "grossAmount")]
    #[serde(default)]
    pub gross_amount: String,
}

impl ReceiptsGetPosResponseLinesItem {
    pub fn builder() -> ReceiptsGetPosResponseLinesItemBuilder {
        <ReceiptsGetPosResponseLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceiptsGetPosResponseLinesItemBuilder {
    id: Option<String>,
    item_id: Option<String>,
    description: Option<String>,
    quantity: Option<String>,
    unit_price_incl_vat: Option<String>,
    vat_rate_percent: Option<String>,
    net_amount: Option<String>,
    vat_amount: Option<String>,
    gross_amount: Option<String>,
}

impl ReceiptsGetPosResponseLinesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

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

    pub fn net_amount(mut self, value: impl Into<String>) -> Self {
        self.net_amount = Some(value.into());
        self
    }

    pub fn vat_amount(mut self, value: impl Into<String>) -> Self {
        self.vat_amount = Some(value.into());
        self
    }

    pub fn gross_amount(mut self, value: impl Into<String>) -> Self {
        self.gross_amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReceiptsGetPosResponseLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReceiptsGetPosResponseLinesItemBuilder::id)
    /// - [`description`](ReceiptsGetPosResponseLinesItemBuilder::description)
    /// - [`quantity`](ReceiptsGetPosResponseLinesItemBuilder::quantity)
    /// - [`unit_price_incl_vat`](ReceiptsGetPosResponseLinesItemBuilder::unit_price_incl_vat)
    /// - [`vat_rate_percent`](ReceiptsGetPosResponseLinesItemBuilder::vat_rate_percent)
    /// - [`net_amount`](ReceiptsGetPosResponseLinesItemBuilder::net_amount)
    /// - [`vat_amount`](ReceiptsGetPosResponseLinesItemBuilder::vat_amount)
    /// - [`gross_amount`](ReceiptsGetPosResponseLinesItemBuilder::gross_amount)
    pub fn build(self) -> Result<ReceiptsGetPosResponseLinesItem, BuildError> {
        Ok(ReceiptsGetPosResponseLinesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            item_id: self.item_id,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            unit_price_incl_vat: self
                .unit_price_incl_vat
                .ok_or_else(|| BuildError::missing_field("unit_price_incl_vat"))?,
            vat_rate_percent: self
                .vat_rate_percent
                .ok_or_else(|| BuildError::missing_field("vat_rate_percent"))?,
            net_amount: self
                .net_amount
                .ok_or_else(|| BuildError::missing_field("net_amount"))?,
            vat_amount: self
                .vat_amount
                .ok_or_else(|| BuildError::missing_field("vat_amount"))?,
            gross_amount: self
                .gross_amount
                .ok_or_else(|| BuildError::missing_field("gross_amount"))?,
        })
    }
}

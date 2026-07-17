pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1EcommerceOrdersCreateRequestLinesItem {
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub quantity: String,
    #[serde(rename = "unitPriceExclVat")]
    #[serde(default)]
    pub unit_price_excl_vat: String,
    #[serde(rename = "vatRatePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_rate_percent: Option<String>,
}

impl PostV1EcommerceOrdersCreateRequestLinesItem {
    pub fn builder() -> PostV1EcommerceOrdersCreateRequestLinesItemBuilder {
        <PostV1EcommerceOrdersCreateRequestLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1EcommerceOrdersCreateRequestLinesItemBuilder {
    item_id: Option<String>,
    description: Option<String>,
    quantity: Option<String>,
    unit_price_excl_vat: Option<String>,
    vat_rate_percent: Option<String>,
}

impl PostV1EcommerceOrdersCreateRequestLinesItemBuilder {
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

    pub fn unit_price_excl_vat(mut self, value: impl Into<String>) -> Self {
        self.unit_price_excl_vat = Some(value.into());
        self
    }

    pub fn vat_rate_percent(mut self, value: impl Into<String>) -> Self {
        self.vat_rate_percent = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1EcommerceOrdersCreateRequestLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](PostV1EcommerceOrdersCreateRequestLinesItemBuilder::description)
    /// - [`quantity`](PostV1EcommerceOrdersCreateRequestLinesItemBuilder::quantity)
    /// - [`unit_price_excl_vat`](PostV1EcommerceOrdersCreateRequestLinesItemBuilder::unit_price_excl_vat)
    pub fn build(self) -> Result<PostV1EcommerceOrdersCreateRequestLinesItem, BuildError> {
        Ok(PostV1EcommerceOrdersCreateRequestLinesItem {
            item_id: self.item_id,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            unit_price_excl_vat: self
                .unit_price_excl_vat
                .ok_or_else(|| BuildError::missing_field("unit_price_excl_vat"))?,
            vat_rate_percent: self.vat_rate_percent,
        })
    }
}

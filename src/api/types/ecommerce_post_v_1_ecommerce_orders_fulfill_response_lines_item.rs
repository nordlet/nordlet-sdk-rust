pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1EcommerceOrdersFulfillResponseLinesItem {
    #[serde(default)]
    pub id: String,
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
    #[serde(default)]
    pub vat_rate_percent: String,
}

impl PostV1EcommerceOrdersFulfillResponseLinesItem {
    pub fn builder() -> PostV1EcommerceOrdersFulfillResponseLinesItemBuilder {
        <PostV1EcommerceOrdersFulfillResponseLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1EcommerceOrdersFulfillResponseLinesItemBuilder {
    id: Option<String>,
    item_id: Option<String>,
    description: Option<String>,
    quantity: Option<String>,
    unit_price_excl_vat: Option<String>,
    vat_rate_percent: Option<String>,
}

impl PostV1EcommerceOrdersFulfillResponseLinesItemBuilder {
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

    pub fn unit_price_excl_vat(mut self, value: impl Into<String>) -> Self {
        self.unit_price_excl_vat = Some(value.into());
        self
    }

    pub fn vat_rate_percent(mut self, value: impl Into<String>) -> Self {
        self.vat_rate_percent = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1EcommerceOrdersFulfillResponseLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1EcommerceOrdersFulfillResponseLinesItemBuilder::id)
    /// - [`description`](PostV1EcommerceOrdersFulfillResponseLinesItemBuilder::description)
    /// - [`quantity`](PostV1EcommerceOrdersFulfillResponseLinesItemBuilder::quantity)
    /// - [`unit_price_excl_vat`](PostV1EcommerceOrdersFulfillResponseLinesItemBuilder::unit_price_excl_vat)
    /// - [`vat_rate_percent`](PostV1EcommerceOrdersFulfillResponseLinesItemBuilder::vat_rate_percent)
    pub fn build(self) -> Result<PostV1EcommerceOrdersFulfillResponseLinesItem, BuildError> {
        Ok(PostV1EcommerceOrdersFulfillResponseLinesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
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
            vat_rate_percent: self
                .vat_rate_percent
                .ok_or_else(|| BuildError::missing_field("vat_rate_percent"))?,
        })
    }
}

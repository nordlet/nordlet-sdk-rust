pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesActsUpdateResponseLinesItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub unit: String,
    #[serde(default)]
    pub quantity: String,
    #[serde(rename = "unitPriceExclVat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price_excl_vat: Option<String>,
    #[serde(rename = "lineNet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_net: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    pub sort_order: i64,
}

impl PostV1SalesActsUpdateResponseLinesItem {
    pub fn builder() -> PostV1SalesActsUpdateResponseLinesItemBuilder {
        <PostV1SalesActsUpdateResponseLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesActsUpdateResponseLinesItemBuilder {
    id: Option<String>,
    item_id: Option<String>,
    description: Option<String>,
    unit: Option<String>,
    quantity: Option<String>,
    unit_price_excl_vat: Option<String>,
    line_net: Option<String>,
    sort_order: Option<i64>,
}

impl PostV1SalesActsUpdateResponseLinesItemBuilder {
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

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
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

    pub fn line_net(mut self, value: impl Into<String>) -> Self {
        self.line_net = Some(value.into());
        self
    }

    pub fn sort_order(mut self, value: i64) -> Self {
        self.sort_order = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesActsUpdateResponseLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1SalesActsUpdateResponseLinesItemBuilder::id)
    /// - [`description`](PostV1SalesActsUpdateResponseLinesItemBuilder::description)
    /// - [`unit`](PostV1SalesActsUpdateResponseLinesItemBuilder::unit)
    /// - [`quantity`](PostV1SalesActsUpdateResponseLinesItemBuilder::quantity)
    /// - [`sort_order`](PostV1SalesActsUpdateResponseLinesItemBuilder::sort_order)
    pub fn build(self) -> Result<PostV1SalesActsUpdateResponseLinesItem, BuildError> {
        Ok(PostV1SalesActsUpdateResponseLinesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            item_id: self.item_id,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            unit_price_excl_vat: self.unit_price_excl_vat,
            line_net: self.line_net,
            sort_order: self
                .sort_order
                .ok_or_else(|| BuildError::missing_field("sort_order"))?,
        })
    }
}

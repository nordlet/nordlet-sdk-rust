pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesActsCreateRequestLinesItem {
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default)]
    pub quantity: String,
    #[serde(rename = "unitPriceExclVat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price_excl_vat: Option<String>,
}

impl PostV1SalesActsCreateRequestLinesItem {
    pub fn builder() -> PostV1SalesActsCreateRequestLinesItemBuilder {
        <PostV1SalesActsCreateRequestLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesActsCreateRequestLinesItemBuilder {
    item_id: Option<String>,
    description: Option<String>,
    unit: Option<String>,
    quantity: Option<String>,
    unit_price_excl_vat: Option<String>,
}

impl PostV1SalesActsCreateRequestLinesItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1SalesActsCreateRequestLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`quantity`](PostV1SalesActsCreateRequestLinesItemBuilder::quantity)
    pub fn build(self) -> Result<PostV1SalesActsCreateRequestLinesItem, BuildError> {
        Ok(PostV1SalesActsCreateRequestLinesItem {
            item_id: self.item_id,
            description: self.description,
            unit: self.unit,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            unit_price_excl_vat: self.unit_price_excl_vat,
        })
    }
}

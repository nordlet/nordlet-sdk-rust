pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PurchasesReceiptsGetResponseLinesItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "orderLineId")]
    #[serde(default)]
    pub order_line_id: String,
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(default)]
    pub quantity: String,
    #[serde(rename = "unitCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_cost: Option<String>,
    #[serde(rename = "stockMovementId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock_movement_id: Option<String>,
}

impl PostV1PurchasesReceiptsGetResponseLinesItem {
    pub fn builder() -> PostV1PurchasesReceiptsGetResponseLinesItemBuilder {
        <PostV1PurchasesReceiptsGetResponseLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesReceiptsGetResponseLinesItemBuilder {
    id: Option<String>,
    order_line_id: Option<String>,
    item_id: Option<String>,
    quantity: Option<String>,
    unit_cost: Option<String>,
    stock_movement_id: Option<String>,
}

impl PostV1PurchasesReceiptsGetResponseLinesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn order_line_id(mut self, value: impl Into<String>) -> Self {
        self.order_line_id = Some(value.into());
        self
    }

    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    pub fn unit_cost(mut self, value: impl Into<String>) -> Self {
        self.unit_cost = Some(value.into());
        self
    }

    pub fn stock_movement_id(mut self, value: impl Into<String>) -> Self {
        self.stock_movement_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesReceiptsGetResponseLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PurchasesReceiptsGetResponseLinesItemBuilder::id)
    /// - [`order_line_id`](PostV1PurchasesReceiptsGetResponseLinesItemBuilder::order_line_id)
    /// - [`quantity`](PostV1PurchasesReceiptsGetResponseLinesItemBuilder::quantity)
    pub fn build(self) -> Result<PostV1PurchasesReceiptsGetResponseLinesItem, BuildError> {
        Ok(PostV1PurchasesReceiptsGetResponseLinesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            order_line_id: self
                .order_line_id
                .ok_or_else(|| BuildError::missing_field("order_line_id"))?,
            item_id: self.item_id,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            unit_cost: self.unit_cost,
            stock_movement_id: self.stock_movement_id,
        })
    }
}

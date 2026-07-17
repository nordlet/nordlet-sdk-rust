pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsWriteOffActsResponseRowsItem {
    #[serde(rename = "movementId")]
    #[serde(default)]
    pub movement_id: String,
    #[serde(default)]
    pub date: String,
    #[serde(rename = "documentType")]
    #[serde(default)]
    pub document_type: String,
    #[serde(rename = "itemName")]
    #[serde(default)]
    pub item_name: String,
    #[serde(rename = "warehouseCode")]
    #[serde(default)]
    pub warehouse_code: String,
    #[serde(default)]
    pub quantity: String,
    #[serde(rename = "totalCost")]
    #[serde(default)]
    pub total_cost: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1ReportsWriteOffActsResponseRowsItem {
    pub fn builder() -> PostV1ReportsWriteOffActsResponseRowsItemBuilder {
        <PostV1ReportsWriteOffActsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsWriteOffActsResponseRowsItemBuilder {
    movement_id: Option<String>,
    date: Option<String>,
    document_type: Option<String>,
    item_name: Option<String>,
    warehouse_code: Option<String>,
    quantity: Option<String>,
    total_cost: Option<String>,
    notes: Option<String>,
}

impl PostV1ReportsWriteOffActsResponseRowsItemBuilder {
    pub fn movement_id(mut self, value: impl Into<String>) -> Self {
        self.movement_id = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn document_type(mut self, value: impl Into<String>) -> Self {
        self.document_type = Some(value.into());
        self
    }

    pub fn item_name(mut self, value: impl Into<String>) -> Self {
        self.item_name = Some(value.into());
        self
    }

    pub fn warehouse_code(mut self, value: impl Into<String>) -> Self {
        self.warehouse_code = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    pub fn total_cost(mut self, value: impl Into<String>) -> Self {
        self.total_cost = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsWriteOffActsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`movement_id`](PostV1ReportsWriteOffActsResponseRowsItemBuilder::movement_id)
    /// - [`date`](PostV1ReportsWriteOffActsResponseRowsItemBuilder::date)
    /// - [`document_type`](PostV1ReportsWriteOffActsResponseRowsItemBuilder::document_type)
    /// - [`item_name`](PostV1ReportsWriteOffActsResponseRowsItemBuilder::item_name)
    /// - [`warehouse_code`](PostV1ReportsWriteOffActsResponseRowsItemBuilder::warehouse_code)
    /// - [`quantity`](PostV1ReportsWriteOffActsResponseRowsItemBuilder::quantity)
    /// - [`total_cost`](PostV1ReportsWriteOffActsResponseRowsItemBuilder::total_cost)
    pub fn build(self) -> Result<PostV1ReportsWriteOffActsResponseRowsItem, BuildError> {
        Ok(PostV1ReportsWriteOffActsResponseRowsItem {
            movement_id: self
                .movement_id
                .ok_or_else(|| BuildError::missing_field("movement_id"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            document_type: self
                .document_type
                .ok_or_else(|| BuildError::missing_field("document_type"))?,
            item_name: self
                .item_name
                .ok_or_else(|| BuildError::missing_field("item_name"))?,
            warehouse_code: self
                .warehouse_code
                .ok_or_else(|| BuildError::missing_field("warehouse_code"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            total_cost: self
                .total_cost
                .ok_or_else(|| BuildError::missing_field("total_cost"))?,
            notes: self.notes,
        })
    }
}

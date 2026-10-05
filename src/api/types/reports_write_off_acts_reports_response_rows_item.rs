pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WriteOffActsReportsResponseRowsItem {
    #[serde(rename = "movementId")]
    #[serde(default)]
    pub movement_id: String,
    #[serde(default)]
    pub date: NaiveDate,
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

impl WriteOffActsReportsResponseRowsItem {
    pub fn builder() -> WriteOffActsReportsResponseRowsItemBuilder {
        <WriteOffActsReportsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WriteOffActsReportsResponseRowsItemBuilder {
    movement_id: Option<String>,
    date: Option<NaiveDate>,
    document_type: Option<String>,
    item_name: Option<String>,
    warehouse_code: Option<String>,
    quantity: Option<String>,
    total_cost: Option<String>,
    notes: Option<String>,
}

impl WriteOffActsReportsResponseRowsItemBuilder {
    pub fn movement_id(mut self, value: impl Into<String>) -> Self {
        self.movement_id = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
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

    /// Consumes the builder and constructs a [`WriteOffActsReportsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`movement_id`](WriteOffActsReportsResponseRowsItemBuilder::movement_id)
    /// - [`date`](WriteOffActsReportsResponseRowsItemBuilder::date)
    /// - [`document_type`](WriteOffActsReportsResponseRowsItemBuilder::document_type)
    /// - [`item_name`](WriteOffActsReportsResponseRowsItemBuilder::item_name)
    /// - [`warehouse_code`](WriteOffActsReportsResponseRowsItemBuilder::warehouse_code)
    /// - [`quantity`](WriteOffActsReportsResponseRowsItemBuilder::quantity)
    /// - [`total_cost`](WriteOffActsReportsResponseRowsItemBuilder::total_cost)
    pub fn build(self) -> Result<WriteOffActsReportsResponseRowsItem, BuildError> {
        Ok(WriteOffActsReportsResponseRowsItem {
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

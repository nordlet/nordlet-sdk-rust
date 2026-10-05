pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BooksValidateMigrationRequestStockItem {
    #[serde(rename = "warehouseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_code: Option<String>,
    #[serde(rename = "itemCode")]
    #[serde(default)]
    pub item_code: String,
    #[serde(default)]
    pub quantity: String,
    #[serde(rename = "unitCost")]
    #[serde(default)]
    pub unit_cost: String,
    #[serde(rename = "lotNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lot_number: Option<String>,
    #[serde(rename = "expiryDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<NaiveDate>,
}

impl BooksValidateMigrationRequestStockItem {
    pub fn builder() -> BooksValidateMigrationRequestStockItemBuilder {
        <BooksValidateMigrationRequestStockItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BooksValidateMigrationRequestStockItemBuilder {
    warehouse_code: Option<String>,
    item_code: Option<String>,
    quantity: Option<String>,
    unit_cost: Option<String>,
    lot_number: Option<String>,
    expiry_date: Option<NaiveDate>,
}

impl BooksValidateMigrationRequestStockItemBuilder {
    pub fn warehouse_code(mut self, value: impl Into<String>) -> Self {
        self.warehouse_code = Some(value.into());
        self
    }

    pub fn item_code(mut self, value: impl Into<String>) -> Self {
        self.item_code = Some(value.into());
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

    pub fn lot_number(mut self, value: impl Into<String>) -> Self {
        self.lot_number = Some(value.into());
        self
    }

    pub fn expiry_date(mut self, value: NaiveDate) -> Self {
        self.expiry_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BooksValidateMigrationRequestStockItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_code`](BooksValidateMigrationRequestStockItemBuilder::item_code)
    /// - [`quantity`](BooksValidateMigrationRequestStockItemBuilder::quantity)
    /// - [`unit_cost`](BooksValidateMigrationRequestStockItemBuilder::unit_cost)
    pub fn build(self) -> Result<BooksValidateMigrationRequestStockItem, BuildError> {
        Ok(BooksValidateMigrationRequestStockItem {
            warehouse_code: self.warehouse_code,
            item_code: self
                .item_code
                .ok_or_else(|| BuildError::missing_field("item_code"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            unit_cost: self
                .unit_cost
                .ok_or_else(|| BuildError::missing_field("unit_cost"))?,
            lot_number: self.lot_number,
            expiry_date: self.expiry_date,
        })
    }
}

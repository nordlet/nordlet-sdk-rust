pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StockMovementsListInventoryRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<StockMovementsListInventoryRequestSortItemDir>,
}

impl StockMovementsListInventoryRequestSortItem {
    pub fn builder() -> StockMovementsListInventoryRequestSortItemBuilder {
        <StockMovementsListInventoryRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StockMovementsListInventoryRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<StockMovementsListInventoryRequestSortItemDir>,
}

impl StockMovementsListInventoryRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: StockMovementsListInventoryRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StockMovementsListInventoryRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](StockMovementsListInventoryRequestSortItemBuilder::field)
    pub fn build(self) -> Result<StockMovementsListInventoryRequestSortItem, BuildError> {
        Ok(StockMovementsListInventoryRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}

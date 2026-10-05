pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WarehousesListInventoryRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<WarehousesListInventoryRequestSortItemDir>,
}

impl WarehousesListInventoryRequestSortItem {
    pub fn builder() -> WarehousesListInventoryRequestSortItemBuilder {
        <WarehousesListInventoryRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WarehousesListInventoryRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<WarehousesListInventoryRequestSortItemDir>,
}

impl WarehousesListInventoryRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: WarehousesListInventoryRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WarehousesListInventoryRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](WarehousesListInventoryRequestSortItemBuilder::field)
    pub fn build(self) -> Result<WarehousesListInventoryRequestSortItem, BuildError> {
        Ok(WarehousesListInventoryRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}

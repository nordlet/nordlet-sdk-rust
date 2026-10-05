pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LotsListInventoryRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<LotsListInventoryRequestSortItemDir>,
}

impl LotsListInventoryRequestSortItem {
    pub fn builder() -> LotsListInventoryRequestSortItemBuilder {
        <LotsListInventoryRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LotsListInventoryRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<LotsListInventoryRequestSortItemDir>,
}

impl LotsListInventoryRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: LotsListInventoryRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LotsListInventoryRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](LotsListInventoryRequestSortItemBuilder::field)
    pub fn build(self) -> Result<LotsListInventoryRequestSortItem, BuildError> {
        Ok(LotsListInventoryRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}

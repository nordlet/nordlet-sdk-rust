pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LandedCostsListInventoryRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<LandedCostsListInventoryRequestSortItemDir>,
}

impl LandedCostsListInventoryRequestSortItem {
    pub fn builder() -> LandedCostsListInventoryRequestSortItemBuilder {
        <LandedCostsListInventoryRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LandedCostsListInventoryRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<LandedCostsListInventoryRequestSortItemDir>,
}

impl LandedCostsListInventoryRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: LandedCostsListInventoryRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LandedCostsListInventoryRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](LandedCostsListInventoryRequestSortItemBuilder::field)
    pub fn build(self) -> Result<LandedCostsListInventoryRequestSortItem, BuildError> {
        Ok(LandedCostsListInventoryRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}

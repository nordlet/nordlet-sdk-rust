pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReorderRulesListInventoryRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<ReorderRulesListInventoryRequestSortItemDir>,
}

impl ReorderRulesListInventoryRequestSortItem {
    pub fn builder() -> ReorderRulesListInventoryRequestSortItemBuilder {
        <ReorderRulesListInventoryRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReorderRulesListInventoryRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<ReorderRulesListInventoryRequestSortItemDir>,
}

impl ReorderRulesListInventoryRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: ReorderRulesListInventoryRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReorderRulesListInventoryRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ReorderRulesListInventoryRequestSortItemBuilder::field)
    pub fn build(self) -> Result<ReorderRulesListInventoryRequestSortItem, BuildError> {
        Ok(ReorderRulesListInventoryRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}

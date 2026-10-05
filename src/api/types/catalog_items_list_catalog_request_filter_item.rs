pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ItemsListCatalogRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: ItemsListCatalogRequestFilterItemOp,
    pub value: ItemsListCatalogRequestFilterItemValue,
}

impl ItemsListCatalogRequestFilterItem {
    pub fn builder() -> ItemsListCatalogRequestFilterItemBuilder {
        <ItemsListCatalogRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsListCatalogRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<ItemsListCatalogRequestFilterItemOp>,
    value: Option<ItemsListCatalogRequestFilterItemValue>,
}

impl ItemsListCatalogRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: ItemsListCatalogRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: ItemsListCatalogRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ItemsListCatalogRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ItemsListCatalogRequestFilterItemBuilder::field)
    /// - [`op`](ItemsListCatalogRequestFilterItemBuilder::op)
    /// - [`value`](ItemsListCatalogRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<ItemsListCatalogRequestFilterItem, BuildError> {
        Ok(ItemsListCatalogRequestFilterItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            op: self.op.ok_or_else(|| BuildError::missing_field("op"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}

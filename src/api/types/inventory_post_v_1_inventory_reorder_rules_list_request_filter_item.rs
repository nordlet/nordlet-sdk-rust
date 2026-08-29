pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1InventoryReorderRulesListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1InventoryReorderRulesListRequestFilterItemOp,
    pub value: PostV1InventoryReorderRulesListRequestFilterItemValue,
}

impl PostV1InventoryReorderRulesListRequestFilterItem {
    pub fn builder() -> PostV1InventoryReorderRulesListRequestFilterItemBuilder {
        <PostV1InventoryReorderRulesListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryReorderRulesListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1InventoryReorderRulesListRequestFilterItemOp>,
    value: Option<PostV1InventoryReorderRulesListRequestFilterItemValue>,
}

impl PostV1InventoryReorderRulesListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1InventoryReorderRulesListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1InventoryReorderRulesListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryReorderRulesListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1InventoryReorderRulesListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1InventoryReorderRulesListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1InventoryReorderRulesListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1InventoryReorderRulesListRequestFilterItem, BuildError> {
        Ok(PostV1InventoryReorderRulesListRequestFilterItem {
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

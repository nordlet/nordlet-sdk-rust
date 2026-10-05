pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReorderRulesListInventoryRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: ReorderRulesListInventoryRequestFilterItemOp,
    pub value: ReorderRulesListInventoryRequestFilterItemValue,
}

impl ReorderRulesListInventoryRequestFilterItem {
    pub fn builder() -> ReorderRulesListInventoryRequestFilterItemBuilder {
        <ReorderRulesListInventoryRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReorderRulesListInventoryRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<ReorderRulesListInventoryRequestFilterItemOp>,
    value: Option<ReorderRulesListInventoryRequestFilterItemValue>,
}

impl ReorderRulesListInventoryRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: ReorderRulesListInventoryRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: ReorderRulesListInventoryRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReorderRulesListInventoryRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ReorderRulesListInventoryRequestFilterItemBuilder::field)
    /// - [`op`](ReorderRulesListInventoryRequestFilterItemBuilder::op)
    /// - [`value`](ReorderRulesListInventoryRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<ReorderRulesListInventoryRequestFilterItem, BuildError> {
        Ok(ReorderRulesListInventoryRequestFilterItem {
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

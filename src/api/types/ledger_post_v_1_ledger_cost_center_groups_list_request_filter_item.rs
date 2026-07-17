pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1LedgerCostCenterGroupsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1LedgerCostCenterGroupsListRequestFilterItemOp,
    pub value: PostV1LedgerCostCenterGroupsListRequestFilterItemValue,
}

impl PostV1LedgerCostCenterGroupsListRequestFilterItem {
    pub fn builder() -> PostV1LedgerCostCenterGroupsListRequestFilterItemBuilder {
        <PostV1LedgerCostCenterGroupsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerCostCenterGroupsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1LedgerCostCenterGroupsListRequestFilterItemOp>,
    value: Option<PostV1LedgerCostCenterGroupsListRequestFilterItemValue>,
}

impl PostV1LedgerCostCenterGroupsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1LedgerCostCenterGroupsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1LedgerCostCenterGroupsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerCostCenterGroupsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1LedgerCostCenterGroupsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1LedgerCostCenterGroupsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1LedgerCostCenterGroupsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1LedgerCostCenterGroupsListRequestFilterItem, BuildError> {
        Ok(PostV1LedgerCostCenterGroupsListRequestFilterItem {
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

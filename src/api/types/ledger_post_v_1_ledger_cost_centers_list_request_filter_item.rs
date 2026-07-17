pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1LedgerCostCentersListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1LedgerCostCentersListRequestFilterItemOp,
    pub value: PostV1LedgerCostCentersListRequestFilterItemValue,
}

impl PostV1LedgerCostCentersListRequestFilterItem {
    pub fn builder() -> PostV1LedgerCostCentersListRequestFilterItemBuilder {
        <PostV1LedgerCostCentersListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerCostCentersListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1LedgerCostCentersListRequestFilterItemOp>,
    value: Option<PostV1LedgerCostCentersListRequestFilterItemValue>,
}

impl PostV1LedgerCostCentersListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1LedgerCostCentersListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1LedgerCostCentersListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerCostCentersListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1LedgerCostCentersListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1LedgerCostCentersListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1LedgerCostCentersListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1LedgerCostCentersListRequestFilterItem, BuildError> {
        Ok(PostV1LedgerCostCentersListRequestFilterItem {
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

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupsListAssetsRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: GroupsListAssetsRequestFilterItemOp,
    pub value: GroupsListAssetsRequestFilterItemValue,
}

impl GroupsListAssetsRequestFilterItem {
    pub fn builder() -> GroupsListAssetsRequestFilterItemBuilder {
        <GroupsListAssetsRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GroupsListAssetsRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<GroupsListAssetsRequestFilterItemOp>,
    value: Option<GroupsListAssetsRequestFilterItemValue>,
}

impl GroupsListAssetsRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: GroupsListAssetsRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: GroupsListAssetsRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GroupsListAssetsRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](GroupsListAssetsRequestFilterItemBuilder::field)
    /// - [`op`](GroupsListAssetsRequestFilterItemBuilder::op)
    /// - [`value`](GroupsListAssetsRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<GroupsListAssetsRequestFilterItem, BuildError> {
        Ok(GroupsListAssetsRequestFilterItem {
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

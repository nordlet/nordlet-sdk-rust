pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1AssetsGroupsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1AssetsGroupsListRequestFilterItemOp,
    pub value: PostV1AssetsGroupsListRequestFilterItemValue,
}

impl PostV1AssetsGroupsListRequestFilterItem {
    pub fn builder() -> PostV1AssetsGroupsListRequestFilterItemBuilder {
        <PostV1AssetsGroupsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AssetsGroupsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1AssetsGroupsListRequestFilterItemOp>,
    value: Option<PostV1AssetsGroupsListRequestFilterItemValue>,
}

impl PostV1AssetsGroupsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1AssetsGroupsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1AssetsGroupsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AssetsGroupsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1AssetsGroupsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1AssetsGroupsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1AssetsGroupsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1AssetsGroupsListRequestFilterItem, BuildError> {
        Ok(PostV1AssetsGroupsListRequestFilterItem {
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

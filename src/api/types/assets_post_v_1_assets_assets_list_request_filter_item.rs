pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1AssetsAssetsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1AssetsAssetsListRequestFilterItemOp,
    pub value: PostV1AssetsAssetsListRequestFilterItemValue,
}

impl PostV1AssetsAssetsListRequestFilterItem {
    pub fn builder() -> PostV1AssetsAssetsListRequestFilterItemBuilder {
        <PostV1AssetsAssetsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AssetsAssetsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1AssetsAssetsListRequestFilterItemOp>,
    value: Option<PostV1AssetsAssetsListRequestFilterItemValue>,
}

impl PostV1AssetsAssetsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1AssetsAssetsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1AssetsAssetsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AssetsAssetsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1AssetsAssetsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1AssetsAssetsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1AssetsAssetsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1AssetsAssetsListRequestFilterItem, BuildError> {
        Ok(PostV1AssetsAssetsListRequestFilterItem {
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

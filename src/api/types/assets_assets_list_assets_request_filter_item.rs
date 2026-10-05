pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssetsListAssetsRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: AssetsListAssetsRequestFilterItemOp,
    pub value: AssetsListAssetsRequestFilterItemValue,
}

impl AssetsListAssetsRequestFilterItem {
    pub fn builder() -> AssetsListAssetsRequestFilterItemBuilder {
        <AssetsListAssetsRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetsListAssetsRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<AssetsListAssetsRequestFilterItemOp>,
    value: Option<AssetsListAssetsRequestFilterItemValue>,
}

impl AssetsListAssetsRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: AssetsListAssetsRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: AssetsListAssetsRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AssetsListAssetsRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](AssetsListAssetsRequestFilterItemBuilder::field)
    /// - [`op`](AssetsListAssetsRequestFilterItemBuilder::op)
    /// - [`value`](AssetsListAssetsRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<AssetsListAssetsRequestFilterItem, BuildError> {
        Ok(AssetsListAssetsRequestFilterItem {
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

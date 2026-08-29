pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1ProductionRoutingsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1ProductionRoutingsListRequestFilterItemOp,
    pub value: PostV1ProductionRoutingsListRequestFilterItemValue,
}

impl PostV1ProductionRoutingsListRequestFilterItem {
    pub fn builder() -> PostV1ProductionRoutingsListRequestFilterItemBuilder {
        <PostV1ProductionRoutingsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionRoutingsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1ProductionRoutingsListRequestFilterItemOp>,
    value: Option<PostV1ProductionRoutingsListRequestFilterItemValue>,
}

impl PostV1ProductionRoutingsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1ProductionRoutingsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1ProductionRoutingsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionRoutingsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ProductionRoutingsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1ProductionRoutingsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1ProductionRoutingsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1ProductionRoutingsListRequestFilterItem, BuildError> {
        Ok(PostV1ProductionRoutingsListRequestFilterItem {
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

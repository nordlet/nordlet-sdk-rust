pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1ProductionBomsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1ProductionBomsListRequestFilterItemOp,
    pub value: PostV1ProductionBomsListRequestFilterItemValue,
}

impl PostV1ProductionBomsListRequestFilterItem {
    pub fn builder() -> PostV1ProductionBomsListRequestFilterItemBuilder {
        <PostV1ProductionBomsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionBomsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1ProductionBomsListRequestFilterItemOp>,
    value: Option<PostV1ProductionBomsListRequestFilterItemValue>,
}

impl PostV1ProductionBomsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1ProductionBomsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1ProductionBomsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionBomsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ProductionBomsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1ProductionBomsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1ProductionBomsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1ProductionBomsListRequestFilterItem, BuildError> {
        Ok(PostV1ProductionBomsListRequestFilterItem {
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

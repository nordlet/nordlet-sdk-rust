pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1ProductionQualityChecksListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1ProductionQualityChecksListRequestFilterItemOp,
    pub value: PostV1ProductionQualityChecksListRequestFilterItemValue,
}

impl PostV1ProductionQualityChecksListRequestFilterItem {
    pub fn builder() -> PostV1ProductionQualityChecksListRequestFilterItemBuilder {
        <PostV1ProductionQualityChecksListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionQualityChecksListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1ProductionQualityChecksListRequestFilterItemOp>,
    value: Option<PostV1ProductionQualityChecksListRequestFilterItemValue>,
}

impl PostV1ProductionQualityChecksListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1ProductionQualityChecksListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1ProductionQualityChecksListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionQualityChecksListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ProductionQualityChecksListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1ProductionQualityChecksListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1ProductionQualityChecksListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1ProductionQualityChecksListRequestFilterItem, BuildError> {
        Ok(PostV1ProductionQualityChecksListRequestFilterItem {
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

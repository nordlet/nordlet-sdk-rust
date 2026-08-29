pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1ProductionWorkCentersListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1ProductionWorkCentersListRequestFilterItemOp,
    pub value: PostV1ProductionWorkCentersListRequestFilterItemValue,
}

impl PostV1ProductionWorkCentersListRequestFilterItem {
    pub fn builder() -> PostV1ProductionWorkCentersListRequestFilterItemBuilder {
        <PostV1ProductionWorkCentersListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionWorkCentersListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1ProductionWorkCentersListRequestFilterItemOp>,
    value: Option<PostV1ProductionWorkCentersListRequestFilterItemValue>,
}

impl PostV1ProductionWorkCentersListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1ProductionWorkCentersListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1ProductionWorkCentersListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionWorkCentersListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ProductionWorkCentersListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1ProductionWorkCentersListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1ProductionWorkCentersListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1ProductionWorkCentersListRequestFilterItem, BuildError> {
        Ok(PostV1ProductionWorkCentersListRequestFilterItem {
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

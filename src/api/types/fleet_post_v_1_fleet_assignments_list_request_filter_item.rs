pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1FleetAssignmentsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1FleetAssignmentsListRequestFilterItemOp,
    pub value: PostV1FleetAssignmentsListRequestFilterItemValue,
}

impl PostV1FleetAssignmentsListRequestFilterItem {
    pub fn builder() -> PostV1FleetAssignmentsListRequestFilterItemBuilder {
        <PostV1FleetAssignmentsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1FleetAssignmentsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1FleetAssignmentsListRequestFilterItemOp>,
    value: Option<PostV1FleetAssignmentsListRequestFilterItemValue>,
}

impl PostV1FleetAssignmentsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1FleetAssignmentsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1FleetAssignmentsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1FleetAssignmentsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1FleetAssignmentsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1FleetAssignmentsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1FleetAssignmentsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1FleetAssignmentsListRequestFilterItem, BuildError> {
        Ok(PostV1FleetAssignmentsListRequestFilterItem {
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

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1ProductionMaintenanceListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1ProductionMaintenanceListRequestFilterItemOp,
    pub value: PostV1ProductionMaintenanceListRequestFilterItemValue,
}

impl PostV1ProductionMaintenanceListRequestFilterItem {
    pub fn builder() -> PostV1ProductionMaintenanceListRequestFilterItemBuilder {
        <PostV1ProductionMaintenanceListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionMaintenanceListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1ProductionMaintenanceListRequestFilterItemOp>,
    value: Option<PostV1ProductionMaintenanceListRequestFilterItemValue>,
}

impl PostV1ProductionMaintenanceListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1ProductionMaintenanceListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1ProductionMaintenanceListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionMaintenanceListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ProductionMaintenanceListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1ProductionMaintenanceListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1ProductionMaintenanceListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1ProductionMaintenanceListRequestFilterItem, BuildError> {
        Ok(PostV1ProductionMaintenanceListRequestFilterItem {
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

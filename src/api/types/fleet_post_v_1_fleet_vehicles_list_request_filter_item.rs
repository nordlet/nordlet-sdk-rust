pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1FleetVehiclesListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1FleetVehiclesListRequestFilterItemOp,
    pub value: PostV1FleetVehiclesListRequestFilterItemValue,
}

impl PostV1FleetVehiclesListRequestFilterItem {
    pub fn builder() -> PostV1FleetVehiclesListRequestFilterItemBuilder {
        <PostV1FleetVehiclesListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1FleetVehiclesListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1FleetVehiclesListRequestFilterItemOp>,
    value: Option<PostV1FleetVehiclesListRequestFilterItemValue>,
}

impl PostV1FleetVehiclesListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1FleetVehiclesListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1FleetVehiclesListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1FleetVehiclesListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1FleetVehiclesListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1FleetVehiclesListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1FleetVehiclesListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1FleetVehiclesListRequestFilterItem, BuildError> {
        Ok(PostV1FleetVehiclesListRequestFilterItem {
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

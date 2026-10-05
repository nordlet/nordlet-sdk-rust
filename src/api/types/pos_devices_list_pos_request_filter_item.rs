pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DevicesListPosRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: DevicesListPosRequestFilterItemOp,
    pub value: DevicesListPosRequestFilterItemValue,
}

impl DevicesListPosRequestFilterItem {
    pub fn builder() -> DevicesListPosRequestFilterItemBuilder {
        <DevicesListPosRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DevicesListPosRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<DevicesListPosRequestFilterItemOp>,
    value: Option<DevicesListPosRequestFilterItemValue>,
}

impl DevicesListPosRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: DevicesListPosRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: DevicesListPosRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DevicesListPosRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](DevicesListPosRequestFilterItemBuilder::field)
    /// - [`op`](DevicesListPosRequestFilterItemBuilder::op)
    /// - [`value`](DevicesListPosRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<DevicesListPosRequestFilterItem, BuildError> {
        Ok(DevicesListPosRequestFilterItem {
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

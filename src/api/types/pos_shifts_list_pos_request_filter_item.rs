pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShiftsListPosRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: ShiftsListPosRequestFilterItemOp,
    pub value: ShiftsListPosRequestFilterItemValue,
}

impl ShiftsListPosRequestFilterItem {
    pub fn builder() -> ShiftsListPosRequestFilterItemBuilder {
        <ShiftsListPosRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ShiftsListPosRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<ShiftsListPosRequestFilterItemOp>,
    value: Option<ShiftsListPosRequestFilterItemValue>,
}

impl ShiftsListPosRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: ShiftsListPosRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: ShiftsListPosRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ShiftsListPosRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ShiftsListPosRequestFilterItemBuilder::field)
    /// - [`op`](ShiftsListPosRequestFilterItemBuilder::op)
    /// - [`value`](ShiftsListPosRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<ShiftsListPosRequestFilterItem, BuildError> {
        Ok(ShiftsListPosRequestFilterItem {
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

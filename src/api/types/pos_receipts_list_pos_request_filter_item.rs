pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReceiptsListPosRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: ReceiptsListPosRequestFilterItemOp,
    pub value: ReceiptsListPosRequestFilterItemValue,
}

impl ReceiptsListPosRequestFilterItem {
    pub fn builder() -> ReceiptsListPosRequestFilterItemBuilder {
        <ReceiptsListPosRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceiptsListPosRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<ReceiptsListPosRequestFilterItemOp>,
    value: Option<ReceiptsListPosRequestFilterItemValue>,
}

impl ReceiptsListPosRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: ReceiptsListPosRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: ReceiptsListPosRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceiptsListPosRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ReceiptsListPosRequestFilterItemBuilder::field)
    /// - [`op`](ReceiptsListPosRequestFilterItemBuilder::op)
    /// - [`value`](ReceiptsListPosRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<ReceiptsListPosRequestFilterItem, BuildError> {
        Ok(ReceiptsListPosRequestFilterItem {
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

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PositionsListHrRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PositionsListHrRequestFilterItemOp,
    pub value: PositionsListHrRequestFilterItemValue,
}

impl PositionsListHrRequestFilterItem {
    pub fn builder() -> PositionsListHrRequestFilterItemBuilder {
        <PositionsListHrRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PositionsListHrRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PositionsListHrRequestFilterItemOp>,
    value: Option<PositionsListHrRequestFilterItemValue>,
}

impl PositionsListHrRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PositionsListHrRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PositionsListHrRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PositionsListHrRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PositionsListHrRequestFilterItemBuilder::field)
    /// - [`op`](PositionsListHrRequestFilterItemBuilder::op)
    /// - [`value`](PositionsListHrRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PositionsListHrRequestFilterItem, BuildError> {
        Ok(PositionsListHrRequestFilterItem {
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

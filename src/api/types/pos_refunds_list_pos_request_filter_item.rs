pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RefundsListPosRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: RefundsListPosRequestFilterItemOp,
    pub value: RefundsListPosRequestFilterItemValue,
}

impl RefundsListPosRequestFilterItem {
    pub fn builder() -> RefundsListPosRequestFilterItemBuilder {
        <RefundsListPosRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefundsListPosRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<RefundsListPosRequestFilterItemOp>,
    value: Option<RefundsListPosRequestFilterItemValue>,
}

impl RefundsListPosRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: RefundsListPosRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: RefundsListPosRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RefundsListPosRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](RefundsListPosRequestFilterItemBuilder::field)
    /// - [`op`](RefundsListPosRequestFilterItemBuilder::op)
    /// - [`value`](RefundsListPosRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<RefundsListPosRequestFilterItem, BuildError> {
        Ok(RefundsListPosRequestFilterItem {
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

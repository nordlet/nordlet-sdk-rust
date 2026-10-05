pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SettlementsListBankRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: SettlementsListBankRequestFilterItemOp,
    pub value: SettlementsListBankRequestFilterItemValue,
}

impl SettlementsListBankRequestFilterItem {
    pub fn builder() -> SettlementsListBankRequestFilterItemBuilder {
        <SettlementsListBankRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettlementsListBankRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<SettlementsListBankRequestFilterItemOp>,
    value: Option<SettlementsListBankRequestFilterItemValue>,
}

impl SettlementsListBankRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: SettlementsListBankRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: SettlementsListBankRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SettlementsListBankRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](SettlementsListBankRequestFilterItemBuilder::field)
    /// - [`op`](SettlementsListBankRequestFilterItemBuilder::op)
    /// - [`value`](SettlementsListBankRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<SettlementsListBankRequestFilterItem, BuildError> {
        Ok(SettlementsListBankRequestFilterItem {
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

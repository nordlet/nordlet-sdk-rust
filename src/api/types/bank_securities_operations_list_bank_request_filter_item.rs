pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecuritiesOperationsListBankRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: SecuritiesOperationsListBankRequestFilterItemOp,
    pub value: SecuritiesOperationsListBankRequestFilterItemValue,
}

impl SecuritiesOperationsListBankRequestFilterItem {
    pub fn builder() -> SecuritiesOperationsListBankRequestFilterItemBuilder {
        <SecuritiesOperationsListBankRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SecuritiesOperationsListBankRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<SecuritiesOperationsListBankRequestFilterItemOp>,
    value: Option<SecuritiesOperationsListBankRequestFilterItemValue>,
}

impl SecuritiesOperationsListBankRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: SecuritiesOperationsListBankRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: SecuritiesOperationsListBankRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SecuritiesOperationsListBankRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](SecuritiesOperationsListBankRequestFilterItemBuilder::field)
    /// - [`op`](SecuritiesOperationsListBankRequestFilterItemBuilder::op)
    /// - [`value`](SecuritiesOperationsListBankRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<SecuritiesOperationsListBankRequestFilterItem, BuildError> {
        Ok(SecuritiesOperationsListBankRequestFilterItem {
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

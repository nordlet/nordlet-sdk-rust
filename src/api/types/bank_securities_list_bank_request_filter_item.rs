pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecuritiesListBankRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: SecuritiesListBankRequestFilterItemOp,
    pub value: SecuritiesListBankRequestFilterItemValue,
}

impl SecuritiesListBankRequestFilterItem {
    pub fn builder() -> SecuritiesListBankRequestFilterItemBuilder {
        <SecuritiesListBankRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SecuritiesListBankRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<SecuritiesListBankRequestFilterItemOp>,
    value: Option<SecuritiesListBankRequestFilterItemValue>,
}

impl SecuritiesListBankRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: SecuritiesListBankRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: SecuritiesListBankRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SecuritiesListBankRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](SecuritiesListBankRequestFilterItemBuilder::field)
    /// - [`op`](SecuritiesListBankRequestFilterItemBuilder::op)
    /// - [`value`](SecuritiesListBankRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<SecuritiesListBankRequestFilterItem, BuildError> {
        Ok(SecuritiesListBankRequestFilterItem {
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

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MandatesListBankRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: MandatesListBankRequestFilterItemOp,
    pub value: MandatesListBankRequestFilterItemValue,
}

impl MandatesListBankRequestFilterItem {
    pub fn builder() -> MandatesListBankRequestFilterItemBuilder {
        <MandatesListBankRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MandatesListBankRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<MandatesListBankRequestFilterItemOp>,
    value: Option<MandatesListBankRequestFilterItemValue>,
}

impl MandatesListBankRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: MandatesListBankRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: MandatesListBankRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MandatesListBankRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](MandatesListBankRequestFilterItemBuilder::field)
    /// - [`op`](MandatesListBankRequestFilterItemBuilder::op)
    /// - [`value`](MandatesListBankRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<MandatesListBankRequestFilterItem, BuildError> {
        Ok(MandatesListBankRequestFilterItem {
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

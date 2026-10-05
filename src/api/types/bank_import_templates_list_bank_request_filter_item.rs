pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportTemplatesListBankRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: ImportTemplatesListBankRequestFilterItemOp,
    pub value: ImportTemplatesListBankRequestFilterItemValue,
}

impl ImportTemplatesListBankRequestFilterItem {
    pub fn builder() -> ImportTemplatesListBankRequestFilterItemBuilder {
        <ImportTemplatesListBankRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportTemplatesListBankRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<ImportTemplatesListBankRequestFilterItemOp>,
    value: Option<ImportTemplatesListBankRequestFilterItemValue>,
}

impl ImportTemplatesListBankRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: ImportTemplatesListBankRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: ImportTemplatesListBankRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ImportTemplatesListBankRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ImportTemplatesListBankRequestFilterItemBuilder::field)
    /// - [`op`](ImportTemplatesListBankRequestFilterItemBuilder::op)
    /// - [`value`](ImportTemplatesListBankRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<ImportTemplatesListBankRequestFilterItem, BuildError> {
        Ok(ImportTemplatesListBankRequestFilterItem {
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

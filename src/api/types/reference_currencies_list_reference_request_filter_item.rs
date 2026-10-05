pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CurrenciesListReferenceRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: CurrenciesListReferenceRequestFilterItemOp,
    pub value: CurrenciesListReferenceRequestFilterItemValue,
}

impl CurrenciesListReferenceRequestFilterItem {
    pub fn builder() -> CurrenciesListReferenceRequestFilterItemBuilder {
        <CurrenciesListReferenceRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CurrenciesListReferenceRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<CurrenciesListReferenceRequestFilterItemOp>,
    value: Option<CurrenciesListReferenceRequestFilterItemValue>,
}

impl CurrenciesListReferenceRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: CurrenciesListReferenceRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: CurrenciesListReferenceRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CurrenciesListReferenceRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](CurrenciesListReferenceRequestFilterItemBuilder::field)
    /// - [`op`](CurrenciesListReferenceRequestFilterItemBuilder::op)
    /// - [`value`](CurrenciesListReferenceRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<CurrenciesListReferenceRequestFilterItem, BuildError> {
        Ok(CurrenciesListReferenceRequestFilterItem {
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

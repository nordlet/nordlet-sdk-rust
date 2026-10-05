pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExchangeRatesListReferenceRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: ExchangeRatesListReferenceRequestFilterItemOp,
    pub value: ExchangeRatesListReferenceRequestFilterItemValue,
}

impl ExchangeRatesListReferenceRequestFilterItem {
    pub fn builder() -> ExchangeRatesListReferenceRequestFilterItemBuilder {
        <ExchangeRatesListReferenceRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExchangeRatesListReferenceRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<ExchangeRatesListReferenceRequestFilterItemOp>,
    value: Option<ExchangeRatesListReferenceRequestFilterItemValue>,
}

impl ExchangeRatesListReferenceRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: ExchangeRatesListReferenceRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: ExchangeRatesListReferenceRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExchangeRatesListReferenceRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ExchangeRatesListReferenceRequestFilterItemBuilder::field)
    /// - [`op`](ExchangeRatesListReferenceRequestFilterItemBuilder::op)
    /// - [`value`](ExchangeRatesListReferenceRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<ExchangeRatesListReferenceRequestFilterItem, BuildError> {
        Ok(ExchangeRatesListReferenceRequestFilterItem {
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

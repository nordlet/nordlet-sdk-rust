pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExchangeRatesOverridesListReferenceRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: ExchangeRatesOverridesListReferenceRequestFilterItemOp,
    pub value: ExchangeRatesOverridesListReferenceRequestFilterItemValue,
}

impl ExchangeRatesOverridesListReferenceRequestFilterItem {
    pub fn builder() -> ExchangeRatesOverridesListReferenceRequestFilterItemBuilder {
        <ExchangeRatesOverridesListReferenceRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExchangeRatesOverridesListReferenceRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<ExchangeRatesOverridesListReferenceRequestFilterItemOp>,
    value: Option<ExchangeRatesOverridesListReferenceRequestFilterItemValue>,
}

impl ExchangeRatesOverridesListReferenceRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: ExchangeRatesOverridesListReferenceRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(
        mut self,
        value: ExchangeRatesOverridesListReferenceRequestFilterItemValue,
    ) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExchangeRatesOverridesListReferenceRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ExchangeRatesOverridesListReferenceRequestFilterItemBuilder::field)
    /// - [`op`](ExchangeRatesOverridesListReferenceRequestFilterItemBuilder::op)
    /// - [`value`](ExchangeRatesOverridesListReferenceRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<ExchangeRatesOverridesListReferenceRequestFilterItem, BuildError> {
        Ok(ExchangeRatesOverridesListReferenceRequestFilterItem {
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

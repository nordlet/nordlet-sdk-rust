pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1ReferenceExchangeRatesListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1ReferenceExchangeRatesListRequestFilterItemOp,
    pub value: PostV1ReferenceExchangeRatesListRequestFilterItemValue,
}

impl PostV1ReferenceExchangeRatesListRequestFilterItem {
    pub fn builder() -> PostV1ReferenceExchangeRatesListRequestFilterItemBuilder {
        <PostV1ReferenceExchangeRatesListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceExchangeRatesListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1ReferenceExchangeRatesListRequestFilterItemOp>,
    value: Option<PostV1ReferenceExchangeRatesListRequestFilterItemValue>,
}

impl PostV1ReferenceExchangeRatesListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1ReferenceExchangeRatesListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1ReferenceExchangeRatesListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceExchangeRatesListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ReferenceExchangeRatesListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1ReferenceExchangeRatesListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1ReferenceExchangeRatesListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1ReferenceExchangeRatesListRequestFilterItem, BuildError> {
        Ok(PostV1ReferenceExchangeRatesListRequestFilterItem {
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

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1ReferenceExchangeRatesOverridesListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1ReferenceExchangeRatesOverridesListRequestFilterItemOp,
    pub value: PostV1ReferenceExchangeRatesOverridesListRequestFilterItemValue,
}

impl PostV1ReferenceExchangeRatesOverridesListRequestFilterItem {
    pub fn builder() -> PostV1ReferenceExchangeRatesOverridesListRequestFilterItemBuilder {
        <PostV1ReferenceExchangeRatesOverridesListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceExchangeRatesOverridesListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1ReferenceExchangeRatesOverridesListRequestFilterItemOp>,
    value: Option<PostV1ReferenceExchangeRatesOverridesListRequestFilterItemValue>,
}

impl PostV1ReferenceExchangeRatesOverridesListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(
        mut self,
        value: PostV1ReferenceExchangeRatesOverridesListRequestFilterItemOp,
    ) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(
        mut self,
        value: PostV1ReferenceExchangeRatesOverridesListRequestFilterItemValue,
    ) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceExchangeRatesOverridesListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ReferenceExchangeRatesOverridesListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1ReferenceExchangeRatesOverridesListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1ReferenceExchangeRatesOverridesListRequestFilterItemBuilder::value)
    pub fn build(
        self,
    ) -> Result<PostV1ReferenceExchangeRatesOverridesListRequestFilterItem, BuildError> {
        Ok(PostV1ReferenceExchangeRatesOverridesListRequestFilterItem {
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

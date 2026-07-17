pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1ReferenceCurrenciesListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1ReferenceCurrenciesListRequestFilterItemOp,
    pub value: PostV1ReferenceCurrenciesListRequestFilterItemValue,
}

impl PostV1ReferenceCurrenciesListRequestFilterItem {
    pub fn builder() -> PostV1ReferenceCurrenciesListRequestFilterItemBuilder {
        <PostV1ReferenceCurrenciesListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceCurrenciesListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1ReferenceCurrenciesListRequestFilterItemOp>,
    value: Option<PostV1ReferenceCurrenciesListRequestFilterItemValue>,
}

impl PostV1ReferenceCurrenciesListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1ReferenceCurrenciesListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1ReferenceCurrenciesListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceCurrenciesListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ReferenceCurrenciesListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1ReferenceCurrenciesListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1ReferenceCurrenciesListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1ReferenceCurrenciesListRequestFilterItem, BuildError> {
        Ok(PostV1ReferenceCurrenciesListRequestFilterItem {
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

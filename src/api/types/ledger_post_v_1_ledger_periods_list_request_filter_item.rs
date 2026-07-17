pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1LedgerPeriodsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1LedgerPeriodsListRequestFilterItemOp,
    pub value: PostV1LedgerPeriodsListRequestFilterItemValue,
}

impl PostV1LedgerPeriodsListRequestFilterItem {
    pub fn builder() -> PostV1LedgerPeriodsListRequestFilterItemBuilder {
        <PostV1LedgerPeriodsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerPeriodsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1LedgerPeriodsListRequestFilterItemOp>,
    value: Option<PostV1LedgerPeriodsListRequestFilterItemValue>,
}

impl PostV1LedgerPeriodsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1LedgerPeriodsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1LedgerPeriodsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerPeriodsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1LedgerPeriodsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1LedgerPeriodsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1LedgerPeriodsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1LedgerPeriodsListRequestFilterItem, BuildError> {
        Ok(PostV1LedgerPeriodsListRequestFilterItem {
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

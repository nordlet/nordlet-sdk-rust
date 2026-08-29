pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1PurchasesOrdersListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1PurchasesOrdersListRequestFilterItemOp,
    pub value: PostV1PurchasesOrdersListRequestFilterItemValue,
}

impl PostV1PurchasesOrdersListRequestFilterItem {
    pub fn builder() -> PostV1PurchasesOrdersListRequestFilterItemBuilder {
        <PostV1PurchasesOrdersListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesOrdersListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1PurchasesOrdersListRequestFilterItemOp>,
    value: Option<PostV1PurchasesOrdersListRequestFilterItemValue>,
}

impl PostV1PurchasesOrdersListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1PurchasesOrdersListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1PurchasesOrdersListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesOrdersListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1PurchasesOrdersListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1PurchasesOrdersListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1PurchasesOrdersListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1PurchasesOrdersListRequestFilterItem, BuildError> {
        Ok(PostV1PurchasesOrdersListRequestFilterItem {
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

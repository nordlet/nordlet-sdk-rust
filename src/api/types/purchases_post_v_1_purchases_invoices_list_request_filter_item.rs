pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1PurchasesInvoicesListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1PurchasesInvoicesListRequestFilterItemOp,
    pub value: PostV1PurchasesInvoicesListRequestFilterItemValue,
}

impl PostV1PurchasesInvoicesListRequestFilterItem {
    pub fn builder() -> PostV1PurchasesInvoicesListRequestFilterItemBuilder {
        <PostV1PurchasesInvoicesListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesInvoicesListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1PurchasesInvoicesListRequestFilterItemOp>,
    value: Option<PostV1PurchasesInvoicesListRequestFilterItemValue>,
}

impl PostV1PurchasesInvoicesListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1PurchasesInvoicesListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1PurchasesInvoicesListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesInvoicesListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1PurchasesInvoicesListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1PurchasesInvoicesListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1PurchasesInvoicesListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1PurchasesInvoicesListRequestFilterItem, BuildError> {
        Ok(PostV1PurchasesInvoicesListRequestFilterItem {
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

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1SalesInvoicesListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1SalesInvoicesListRequestFilterItemOp,
    pub value: PostV1SalesInvoicesListRequestFilterItemValue,
}

impl PostV1SalesInvoicesListRequestFilterItem {
    pub fn builder() -> PostV1SalesInvoicesListRequestFilterItemBuilder {
        <PostV1SalesInvoicesListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1SalesInvoicesListRequestFilterItemOp>,
    value: Option<PostV1SalesInvoicesListRequestFilterItemValue>,
}

impl PostV1SalesInvoicesListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1SalesInvoicesListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1SalesInvoicesListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1SalesInvoicesListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1SalesInvoicesListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1SalesInvoicesListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1SalesInvoicesListRequestFilterItem, BuildError> {
        Ok(PostV1SalesInvoicesListRequestFilterItem {
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

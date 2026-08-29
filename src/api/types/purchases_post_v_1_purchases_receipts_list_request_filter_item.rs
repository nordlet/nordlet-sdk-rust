pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1PurchasesReceiptsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1PurchasesReceiptsListRequestFilterItemOp,
    pub value: PostV1PurchasesReceiptsListRequestFilterItemValue,
}

impl PostV1PurchasesReceiptsListRequestFilterItem {
    pub fn builder() -> PostV1PurchasesReceiptsListRequestFilterItemBuilder {
        <PostV1PurchasesReceiptsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesReceiptsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1PurchasesReceiptsListRequestFilterItemOp>,
    value: Option<PostV1PurchasesReceiptsListRequestFilterItemValue>,
}

impl PostV1PurchasesReceiptsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1PurchasesReceiptsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1PurchasesReceiptsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesReceiptsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1PurchasesReceiptsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1PurchasesReceiptsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1PurchasesReceiptsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1PurchasesReceiptsListRequestFilterItem, BuildError> {
        Ok(PostV1PurchasesReceiptsListRequestFilterItem {
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

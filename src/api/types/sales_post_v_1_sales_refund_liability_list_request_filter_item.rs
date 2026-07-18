pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1SalesRefundLiabilityListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1SalesRefundLiabilityListRequestFilterItemOp,
    pub value: PostV1SalesRefundLiabilityListRequestFilterItemValue,
}

impl PostV1SalesRefundLiabilityListRequestFilterItem {
    pub fn builder() -> PostV1SalesRefundLiabilityListRequestFilterItemBuilder {
        <PostV1SalesRefundLiabilityListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesRefundLiabilityListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1SalesRefundLiabilityListRequestFilterItemOp>,
    value: Option<PostV1SalesRefundLiabilityListRequestFilterItemValue>,
}

impl PostV1SalesRefundLiabilityListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1SalesRefundLiabilityListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1SalesRefundLiabilityListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesRefundLiabilityListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1SalesRefundLiabilityListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1SalesRefundLiabilityListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1SalesRefundLiabilityListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1SalesRefundLiabilityListRequestFilterItem, BuildError> {
        Ok(PostV1SalesRefundLiabilityListRequestFilterItem {
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

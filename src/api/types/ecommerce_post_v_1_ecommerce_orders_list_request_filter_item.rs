pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1EcommerceOrdersListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1EcommerceOrdersListRequestFilterItemOp,
    pub value: PostV1EcommerceOrdersListRequestFilterItemValue,
}

impl PostV1EcommerceOrdersListRequestFilterItem {
    pub fn builder() -> PostV1EcommerceOrdersListRequestFilterItemBuilder {
        <PostV1EcommerceOrdersListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1EcommerceOrdersListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1EcommerceOrdersListRequestFilterItemOp>,
    value: Option<PostV1EcommerceOrdersListRequestFilterItemValue>,
}

impl PostV1EcommerceOrdersListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1EcommerceOrdersListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1EcommerceOrdersListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1EcommerceOrdersListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1EcommerceOrdersListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1EcommerceOrdersListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1EcommerceOrdersListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1EcommerceOrdersListRequestFilterItem, BuildError> {
        Ok(PostV1EcommerceOrdersListRequestFilterItem {
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

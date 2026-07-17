pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1ReferenceVatClassifiersListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1ReferenceVatClassifiersListRequestFilterItemOp,
    pub value: PostV1ReferenceVatClassifiersListRequestFilterItemValue,
}

impl PostV1ReferenceVatClassifiersListRequestFilterItem {
    pub fn builder() -> PostV1ReferenceVatClassifiersListRequestFilterItemBuilder {
        <PostV1ReferenceVatClassifiersListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceVatClassifiersListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1ReferenceVatClassifiersListRequestFilterItemOp>,
    value: Option<PostV1ReferenceVatClassifiersListRequestFilterItemValue>,
}

impl PostV1ReferenceVatClassifiersListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1ReferenceVatClassifiersListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1ReferenceVatClassifiersListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceVatClassifiersListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ReferenceVatClassifiersListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1ReferenceVatClassifiersListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1ReferenceVatClassifiersListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1ReferenceVatClassifiersListRequestFilterItem, BuildError> {
        Ok(PostV1ReferenceVatClassifiersListRequestFilterItem {
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

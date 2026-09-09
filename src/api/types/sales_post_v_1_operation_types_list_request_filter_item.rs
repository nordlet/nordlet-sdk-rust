pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1OperationTypesListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1OperationTypesListRequestFilterItemOp,
    pub value: PostV1OperationTypesListRequestFilterItemValue,
}

impl PostV1OperationTypesListRequestFilterItem {
    pub fn builder() -> PostV1OperationTypesListRequestFilterItemBuilder {
        <PostV1OperationTypesListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1OperationTypesListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1OperationTypesListRequestFilterItemOp>,
    value: Option<PostV1OperationTypesListRequestFilterItemValue>,
}

impl PostV1OperationTypesListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1OperationTypesListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1OperationTypesListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1OperationTypesListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1OperationTypesListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1OperationTypesListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1OperationTypesListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1OperationTypesListRequestFilterItem, BuildError> {
        Ok(PostV1OperationTypesListRequestFilterItem {
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

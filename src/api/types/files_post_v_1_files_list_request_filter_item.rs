pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1FilesListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1FilesListRequestFilterItemOp,
    pub value: PostV1FilesListRequestFilterItemValue,
}

impl PostV1FilesListRequestFilterItem {
    pub fn builder() -> PostV1FilesListRequestFilterItemBuilder {
        <PostV1FilesListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1FilesListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1FilesListRequestFilterItemOp>,
    value: Option<PostV1FilesListRequestFilterItemValue>,
}

impl PostV1FilesListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1FilesListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1FilesListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1FilesListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1FilesListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1FilesListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1FilesListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1FilesListRequestFilterItem, BuildError> {
        Ok(PostV1FilesListRequestFilterItem {
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

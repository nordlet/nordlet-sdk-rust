pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1ProjectsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1ProjectsListRequestFilterItemOp,
    pub value: PostV1ProjectsListRequestFilterItemValue,
}

impl PostV1ProjectsListRequestFilterItem {
    pub fn builder() -> PostV1ProjectsListRequestFilterItemBuilder {
        <PostV1ProjectsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProjectsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1ProjectsListRequestFilterItemOp>,
    value: Option<PostV1ProjectsListRequestFilterItemValue>,
}

impl PostV1ProjectsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1ProjectsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1ProjectsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProjectsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ProjectsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1ProjectsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1ProjectsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1ProjectsListRequestFilterItem, BuildError> {
        Ok(PostV1ProjectsListRequestFilterItem {
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

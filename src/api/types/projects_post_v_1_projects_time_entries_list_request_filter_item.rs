pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1ProjectsTimeEntriesListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1ProjectsTimeEntriesListRequestFilterItemOp,
    pub value: PostV1ProjectsTimeEntriesListRequestFilterItemValue,
}

impl PostV1ProjectsTimeEntriesListRequestFilterItem {
    pub fn builder() -> PostV1ProjectsTimeEntriesListRequestFilterItemBuilder {
        <PostV1ProjectsTimeEntriesListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProjectsTimeEntriesListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1ProjectsTimeEntriesListRequestFilterItemOp>,
    value: Option<PostV1ProjectsTimeEntriesListRequestFilterItemValue>,
}

impl PostV1ProjectsTimeEntriesListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1ProjectsTimeEntriesListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1ProjectsTimeEntriesListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProjectsTimeEntriesListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ProjectsTimeEntriesListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1ProjectsTimeEntriesListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1ProjectsTimeEntriesListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1ProjectsTimeEntriesListRequestFilterItem, BuildError> {
        Ok(PostV1ProjectsTimeEntriesListRequestFilterItem {
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

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1DeclarationsSubmissionsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1DeclarationsSubmissionsListRequestFilterItemOp,
    pub value: PostV1DeclarationsSubmissionsListRequestFilterItemValue,
}

impl PostV1DeclarationsSubmissionsListRequestFilterItem {
    pub fn builder() -> PostV1DeclarationsSubmissionsListRequestFilterItemBuilder {
        <PostV1DeclarationsSubmissionsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsSubmissionsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1DeclarationsSubmissionsListRequestFilterItemOp>,
    value: Option<PostV1DeclarationsSubmissionsListRequestFilterItemValue>,
}

impl PostV1DeclarationsSubmissionsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1DeclarationsSubmissionsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1DeclarationsSubmissionsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsSubmissionsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1DeclarationsSubmissionsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1DeclarationsSubmissionsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1DeclarationsSubmissionsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1DeclarationsSubmissionsListRequestFilterItem, BuildError> {
        Ok(PostV1DeclarationsSubmissionsListRequestFilterItem {
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

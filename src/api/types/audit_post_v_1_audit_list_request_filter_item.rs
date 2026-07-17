pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1AuditListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1AuditListRequestFilterItemOp,
    pub value: PostV1AuditListRequestFilterItemValue,
}

impl PostV1AuditListRequestFilterItem {
    pub fn builder() -> PostV1AuditListRequestFilterItemBuilder {
        <PostV1AuditListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AuditListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1AuditListRequestFilterItemOp>,
    value: Option<PostV1AuditListRequestFilterItemValue>,
}

impl PostV1AuditListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1AuditListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1AuditListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AuditListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1AuditListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1AuditListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1AuditListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1AuditListRequestFilterItem, BuildError> {
        Ok(PostV1AuditListRequestFilterItem {
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

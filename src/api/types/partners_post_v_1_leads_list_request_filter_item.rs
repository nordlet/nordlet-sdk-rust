pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1LeadsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1LeadsListRequestFilterItemOp,
    pub value: PostV1LeadsListRequestFilterItemValue,
}

impl PostV1LeadsListRequestFilterItem {
    pub fn builder() -> PostV1LeadsListRequestFilterItemBuilder {
        <PostV1LeadsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LeadsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1LeadsListRequestFilterItemOp>,
    value: Option<PostV1LeadsListRequestFilterItemValue>,
}

impl PostV1LeadsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1LeadsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1LeadsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LeadsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1LeadsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1LeadsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1LeadsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1LeadsListRequestFilterItem, BuildError> {
        Ok(PostV1LeadsListRequestFilterItem {
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

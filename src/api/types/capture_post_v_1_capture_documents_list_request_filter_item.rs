pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1CaptureDocumentsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1CaptureDocumentsListRequestFilterItemOp,
    pub value: PostV1CaptureDocumentsListRequestFilterItemValue,
}

impl PostV1CaptureDocumentsListRequestFilterItem {
    pub fn builder() -> PostV1CaptureDocumentsListRequestFilterItemBuilder {
        <PostV1CaptureDocumentsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CaptureDocumentsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1CaptureDocumentsListRequestFilterItemOp>,
    value: Option<PostV1CaptureDocumentsListRequestFilterItemValue>,
}

impl PostV1CaptureDocumentsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1CaptureDocumentsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1CaptureDocumentsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CaptureDocumentsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1CaptureDocumentsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1CaptureDocumentsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1CaptureDocumentsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1CaptureDocumentsListRequestFilterItem, BuildError> {
        Ok(PostV1CaptureDocumentsListRequestFilterItem {
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

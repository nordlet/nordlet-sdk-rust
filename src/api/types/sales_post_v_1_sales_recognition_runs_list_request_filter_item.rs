pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1SalesRecognitionRunsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1SalesRecognitionRunsListRequestFilterItemOp,
    pub value: PostV1SalesRecognitionRunsListRequestFilterItemValue,
}

impl PostV1SalesRecognitionRunsListRequestFilterItem {
    pub fn builder() -> PostV1SalesRecognitionRunsListRequestFilterItemBuilder {
        <PostV1SalesRecognitionRunsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesRecognitionRunsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1SalesRecognitionRunsListRequestFilterItemOp>,
    value: Option<PostV1SalesRecognitionRunsListRequestFilterItemValue>,
}

impl PostV1SalesRecognitionRunsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1SalesRecognitionRunsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1SalesRecognitionRunsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesRecognitionRunsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1SalesRecognitionRunsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1SalesRecognitionRunsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1SalesRecognitionRunsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1SalesRecognitionRunsListRequestFilterItem, BuildError> {
        Ok(PostV1SalesRecognitionRunsListRequestFilterItem {
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

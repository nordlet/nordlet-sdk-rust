pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1SalesRecognitionSchedulesListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1SalesRecognitionSchedulesListRequestFilterItemOp,
    pub value: PostV1SalesRecognitionSchedulesListRequestFilterItemValue,
}

impl PostV1SalesRecognitionSchedulesListRequestFilterItem {
    pub fn builder() -> PostV1SalesRecognitionSchedulesListRequestFilterItemBuilder {
        <PostV1SalesRecognitionSchedulesListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesRecognitionSchedulesListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1SalesRecognitionSchedulesListRequestFilterItemOp>,
    value: Option<PostV1SalesRecognitionSchedulesListRequestFilterItemValue>,
}

impl PostV1SalesRecognitionSchedulesListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1SalesRecognitionSchedulesListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(
        mut self,
        value: PostV1SalesRecognitionSchedulesListRequestFilterItemValue,
    ) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesRecognitionSchedulesListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1SalesRecognitionSchedulesListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1SalesRecognitionSchedulesListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1SalesRecognitionSchedulesListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1SalesRecognitionSchedulesListRequestFilterItem, BuildError> {
        Ok(PostV1SalesRecognitionSchedulesListRequestFilterItem {
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

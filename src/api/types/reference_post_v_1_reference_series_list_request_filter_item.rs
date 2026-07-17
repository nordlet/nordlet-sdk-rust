pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1ReferenceSeriesListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1ReferenceSeriesListRequestFilterItemOp,
    pub value: PostV1ReferenceSeriesListRequestFilterItemValue,
}

impl PostV1ReferenceSeriesListRequestFilterItem {
    pub fn builder() -> PostV1ReferenceSeriesListRequestFilterItemBuilder {
        <PostV1ReferenceSeriesListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceSeriesListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1ReferenceSeriesListRequestFilterItemOp>,
    value: Option<PostV1ReferenceSeriesListRequestFilterItemValue>,
}

impl PostV1ReferenceSeriesListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1ReferenceSeriesListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1ReferenceSeriesListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceSeriesListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ReferenceSeriesListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1ReferenceSeriesListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1ReferenceSeriesListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1ReferenceSeriesListRequestFilterItem, BuildError> {
        Ok(PostV1ReferenceSeriesListRequestFilterItem {
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

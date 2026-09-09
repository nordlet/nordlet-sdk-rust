pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1DocumentSeriesListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1DocumentSeriesListRequestFilterItemOp,
    pub value: PostV1DocumentSeriesListRequestFilterItemValue,
}

impl PostV1DocumentSeriesListRequestFilterItem {
    pub fn builder() -> PostV1DocumentSeriesListRequestFilterItemBuilder {
        <PostV1DocumentSeriesListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DocumentSeriesListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1DocumentSeriesListRequestFilterItemOp>,
    value: Option<PostV1DocumentSeriesListRequestFilterItemValue>,
}

impl PostV1DocumentSeriesListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1DocumentSeriesListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1DocumentSeriesListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DocumentSeriesListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1DocumentSeriesListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1DocumentSeriesListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1DocumentSeriesListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1DocumentSeriesListRequestFilterItem, BuildError> {
        Ok(PostV1DocumentSeriesListRequestFilterItem {
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

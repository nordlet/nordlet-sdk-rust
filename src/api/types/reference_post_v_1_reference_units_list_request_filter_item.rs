pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1ReferenceUnitsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1ReferenceUnitsListRequestFilterItemOp,
    pub value: PostV1ReferenceUnitsListRequestFilterItemValue,
}

impl PostV1ReferenceUnitsListRequestFilterItem {
    pub fn builder() -> PostV1ReferenceUnitsListRequestFilterItemBuilder {
        <PostV1ReferenceUnitsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceUnitsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1ReferenceUnitsListRequestFilterItemOp>,
    value: Option<PostV1ReferenceUnitsListRequestFilterItemValue>,
}

impl PostV1ReferenceUnitsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1ReferenceUnitsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1ReferenceUnitsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceUnitsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ReferenceUnitsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1ReferenceUnitsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1ReferenceUnitsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1ReferenceUnitsListRequestFilterItem, BuildError> {
        Ok(PostV1ReferenceUnitsListRequestFilterItem {
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

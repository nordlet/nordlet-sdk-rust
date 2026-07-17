pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1ReferenceCnCodesListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1ReferenceCnCodesListRequestFilterItemOp,
    pub value: PostV1ReferenceCnCodesListRequestFilterItemValue,
}

impl PostV1ReferenceCnCodesListRequestFilterItem {
    pub fn builder() -> PostV1ReferenceCnCodesListRequestFilterItemBuilder {
        <PostV1ReferenceCnCodesListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceCnCodesListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1ReferenceCnCodesListRequestFilterItemOp>,
    value: Option<PostV1ReferenceCnCodesListRequestFilterItemValue>,
}

impl PostV1ReferenceCnCodesListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1ReferenceCnCodesListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1ReferenceCnCodesListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceCnCodesListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ReferenceCnCodesListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1ReferenceCnCodesListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1ReferenceCnCodesListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1ReferenceCnCodesListRequestFilterItem, BuildError> {
        Ok(PostV1ReferenceCnCodesListRequestFilterItem {
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

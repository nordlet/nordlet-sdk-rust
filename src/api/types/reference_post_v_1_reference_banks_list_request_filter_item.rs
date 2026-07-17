pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1ReferenceBanksListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1ReferenceBanksListRequestFilterItemOp,
    pub value: PostV1ReferenceBanksListRequestFilterItemValue,
}

impl PostV1ReferenceBanksListRequestFilterItem {
    pub fn builder() -> PostV1ReferenceBanksListRequestFilterItemBuilder {
        <PostV1ReferenceBanksListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceBanksListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1ReferenceBanksListRequestFilterItemOp>,
    value: Option<PostV1ReferenceBanksListRequestFilterItemValue>,
}

impl PostV1ReferenceBanksListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1ReferenceBanksListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1ReferenceBanksListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceBanksListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ReferenceBanksListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1ReferenceBanksListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1ReferenceBanksListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1ReferenceBanksListRequestFilterItem, BuildError> {
        Ok(PostV1ReferenceBanksListRequestFilterItem {
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

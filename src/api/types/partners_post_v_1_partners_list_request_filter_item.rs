pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1PartnersListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1PartnersListRequestFilterItemOp,
    pub value: PostV1PartnersListRequestFilterItemValue,
}

impl PostV1PartnersListRequestFilterItem {
    pub fn builder() -> PostV1PartnersListRequestFilterItemBuilder {
        <PostV1PartnersListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1PartnersListRequestFilterItemOp>,
    value: Option<PostV1PartnersListRequestFilterItemValue>,
}

impl PostV1PartnersListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1PartnersListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1PartnersListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1PartnersListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1PartnersListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1PartnersListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1PartnersListRequestFilterItem, BuildError> {
        Ok(PostV1PartnersListRequestFilterItem {
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

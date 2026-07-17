pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1PartnersContactsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1PartnersContactsListRequestFilterItemOp,
    pub value: PostV1PartnersContactsListRequestFilterItemValue,
}

impl PostV1PartnersContactsListRequestFilterItem {
    pub fn builder() -> PostV1PartnersContactsListRequestFilterItemBuilder {
        <PostV1PartnersContactsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersContactsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1PartnersContactsListRequestFilterItemOp>,
    value: Option<PostV1PartnersContactsListRequestFilterItemValue>,
}

impl PostV1PartnersContactsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1PartnersContactsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1PartnersContactsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersContactsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1PartnersContactsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1PartnersContactsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1PartnersContactsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1PartnersContactsListRequestFilterItem, BuildError> {
        Ok(PostV1PartnersContactsListRequestFilterItem {
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

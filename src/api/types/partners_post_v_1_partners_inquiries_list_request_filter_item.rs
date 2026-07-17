pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1PartnersInquiriesListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1PartnersInquiriesListRequestFilterItemOp,
    pub value: PostV1PartnersInquiriesListRequestFilterItemValue,
}

impl PostV1PartnersInquiriesListRequestFilterItem {
    pub fn builder() -> PostV1PartnersInquiriesListRequestFilterItemBuilder {
        <PostV1PartnersInquiriesListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersInquiriesListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1PartnersInquiriesListRequestFilterItemOp>,
    value: Option<PostV1PartnersInquiriesListRequestFilterItemValue>,
}

impl PostV1PartnersInquiriesListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1PartnersInquiriesListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1PartnersInquiriesListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersInquiriesListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1PartnersInquiriesListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1PartnersInquiriesListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1PartnersInquiriesListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1PartnersInquiriesListRequestFilterItem, BuildError> {
        Ok(PostV1PartnersInquiriesListRequestFilterItem {
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

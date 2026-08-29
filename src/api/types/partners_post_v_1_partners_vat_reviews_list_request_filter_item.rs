pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1PartnersVatReviewsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1PartnersVatReviewsListRequestFilterItemOp,
    pub value: PostV1PartnersVatReviewsListRequestFilterItemValue,
}

impl PostV1PartnersVatReviewsListRequestFilterItem {
    pub fn builder() -> PostV1PartnersVatReviewsListRequestFilterItemBuilder {
        <PostV1PartnersVatReviewsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersVatReviewsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1PartnersVatReviewsListRequestFilterItemOp>,
    value: Option<PostV1PartnersVatReviewsListRequestFilterItemValue>,
}

impl PostV1PartnersVatReviewsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1PartnersVatReviewsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1PartnersVatReviewsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersVatReviewsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1PartnersVatReviewsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1PartnersVatReviewsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1PartnersVatReviewsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1PartnersVatReviewsListRequestFilterItem, BuildError> {
        Ok(PostV1PartnersVatReviewsListRequestFilterItem {
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

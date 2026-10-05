pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VatReviewsListPartnersRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: VatReviewsListPartnersRequestFilterItemOp,
    pub value: VatReviewsListPartnersRequestFilterItemValue,
}

impl VatReviewsListPartnersRequestFilterItem {
    pub fn builder() -> VatReviewsListPartnersRequestFilterItemBuilder {
        <VatReviewsListPartnersRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatReviewsListPartnersRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<VatReviewsListPartnersRequestFilterItemOp>,
    value: Option<VatReviewsListPartnersRequestFilterItemValue>,
}

impl VatReviewsListPartnersRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: VatReviewsListPartnersRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: VatReviewsListPartnersRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VatReviewsListPartnersRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](VatReviewsListPartnersRequestFilterItemBuilder::field)
    /// - [`op`](VatReviewsListPartnersRequestFilterItemBuilder::op)
    /// - [`value`](VatReviewsListPartnersRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<VatReviewsListPartnersRequestFilterItem, BuildError> {
        Ok(VatReviewsListPartnersRequestFilterItem {
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

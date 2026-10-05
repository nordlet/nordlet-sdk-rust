pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InquiriesListPartnersRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: InquiriesListPartnersRequestFilterItemOp,
    pub value: InquiriesListPartnersRequestFilterItemValue,
}

impl InquiriesListPartnersRequestFilterItem {
    pub fn builder() -> InquiriesListPartnersRequestFilterItemBuilder {
        <InquiriesListPartnersRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InquiriesListPartnersRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<InquiriesListPartnersRequestFilterItemOp>,
    value: Option<InquiriesListPartnersRequestFilterItemValue>,
}

impl InquiriesListPartnersRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: InquiriesListPartnersRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: InquiriesListPartnersRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InquiriesListPartnersRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](InquiriesListPartnersRequestFilterItemBuilder::field)
    /// - [`op`](InquiriesListPartnersRequestFilterItemBuilder::op)
    /// - [`value`](InquiriesListPartnersRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<InquiriesListPartnersRequestFilterItem, BuildError> {
        Ok(InquiriesListPartnersRequestFilterItem {
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

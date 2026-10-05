pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VatClassifiersListReferenceRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: VatClassifiersListReferenceRequestFilterItemOp,
    pub value: VatClassifiersListReferenceRequestFilterItemValue,
}

impl VatClassifiersListReferenceRequestFilterItem {
    pub fn builder() -> VatClassifiersListReferenceRequestFilterItemBuilder {
        <VatClassifiersListReferenceRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatClassifiersListReferenceRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<VatClassifiersListReferenceRequestFilterItemOp>,
    value: Option<VatClassifiersListReferenceRequestFilterItemValue>,
}

impl VatClassifiersListReferenceRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: VatClassifiersListReferenceRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: VatClassifiersListReferenceRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VatClassifiersListReferenceRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](VatClassifiersListReferenceRequestFilterItemBuilder::field)
    /// - [`op`](VatClassifiersListReferenceRequestFilterItemBuilder::op)
    /// - [`value`](VatClassifiersListReferenceRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<VatClassifiersListReferenceRequestFilterItem, BuildError> {
        Ok(VatClassifiersListReferenceRequestFilterItem {
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

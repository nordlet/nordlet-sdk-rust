pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnitsListReferenceRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: UnitsListReferenceRequestFilterItemOp,
    pub value: UnitsListReferenceRequestFilterItemValue,
}

impl UnitsListReferenceRequestFilterItem {
    pub fn builder() -> UnitsListReferenceRequestFilterItemBuilder {
        <UnitsListReferenceRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnitsListReferenceRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<UnitsListReferenceRequestFilterItemOp>,
    value: Option<UnitsListReferenceRequestFilterItemValue>,
}

impl UnitsListReferenceRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: UnitsListReferenceRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: UnitsListReferenceRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnitsListReferenceRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](UnitsListReferenceRequestFilterItemBuilder::field)
    /// - [`op`](UnitsListReferenceRequestFilterItemBuilder::op)
    /// - [`value`](UnitsListReferenceRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<UnitsListReferenceRequestFilterItem, BuildError> {
        Ok(UnitsListReferenceRequestFilterItem {
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

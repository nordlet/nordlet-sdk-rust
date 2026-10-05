pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CnCodesListReferenceRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: CnCodesListReferenceRequestFilterItemOp,
    pub value: CnCodesListReferenceRequestFilterItemValue,
}

impl CnCodesListReferenceRequestFilterItem {
    pub fn builder() -> CnCodesListReferenceRequestFilterItemBuilder {
        <CnCodesListReferenceRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CnCodesListReferenceRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<CnCodesListReferenceRequestFilterItemOp>,
    value: Option<CnCodesListReferenceRequestFilterItemValue>,
}

impl CnCodesListReferenceRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: CnCodesListReferenceRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: CnCodesListReferenceRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CnCodesListReferenceRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](CnCodesListReferenceRequestFilterItemBuilder::field)
    /// - [`op`](CnCodesListReferenceRequestFilterItemBuilder::op)
    /// - [`value`](CnCodesListReferenceRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<CnCodesListReferenceRequestFilterItem, BuildError> {
        Ok(CnCodesListReferenceRequestFilterItem {
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

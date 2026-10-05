pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IncapacityCertificatesListHrRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: IncapacityCertificatesListHrRequestFilterItemOp,
    pub value: IncapacityCertificatesListHrRequestFilterItemValue,
}

impl IncapacityCertificatesListHrRequestFilterItem {
    pub fn builder() -> IncapacityCertificatesListHrRequestFilterItemBuilder {
        <IncapacityCertificatesListHrRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IncapacityCertificatesListHrRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<IncapacityCertificatesListHrRequestFilterItemOp>,
    value: Option<IncapacityCertificatesListHrRequestFilterItemValue>,
}

impl IncapacityCertificatesListHrRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: IncapacityCertificatesListHrRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: IncapacityCertificatesListHrRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IncapacityCertificatesListHrRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](IncapacityCertificatesListHrRequestFilterItemBuilder::field)
    /// - [`op`](IncapacityCertificatesListHrRequestFilterItemBuilder::op)
    /// - [`value`](IncapacityCertificatesListHrRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<IncapacityCertificatesListHrRequestFilterItem, BuildError> {
        Ok(IncapacityCertificatesListHrRequestFilterItem {
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

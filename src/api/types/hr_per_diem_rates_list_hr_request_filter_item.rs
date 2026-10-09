pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PerDiemRatesListHrRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PerDiemRatesListHrRequestFilterItemOp,
    pub value: PerDiemRatesListHrRequestFilterItemValue,
}

impl PerDiemRatesListHrRequestFilterItem {
    pub fn builder() -> PerDiemRatesListHrRequestFilterItemBuilder {
        <PerDiemRatesListHrRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PerDiemRatesListHrRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PerDiemRatesListHrRequestFilterItemOp>,
    value: Option<PerDiemRatesListHrRequestFilterItemValue>,
}

impl PerDiemRatesListHrRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PerDiemRatesListHrRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PerDiemRatesListHrRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PerDiemRatesListHrRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PerDiemRatesListHrRequestFilterItemBuilder::field)
    /// - [`op`](PerDiemRatesListHrRequestFilterItemBuilder::op)
    /// - [`value`](PerDiemRatesListHrRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PerDiemRatesListHrRequestFilterItem, BuildError> {
        Ok(PerDiemRatesListHrRequestFilterItem {
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

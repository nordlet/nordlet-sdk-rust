pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReportsListPosRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: ReportsListPosRequestFilterItemOp,
    pub value: ReportsListPosRequestFilterItemValue,
}

impl ReportsListPosRequestFilterItem {
    pub fn builder() -> ReportsListPosRequestFilterItemBuilder {
        <ReportsListPosRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportsListPosRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<ReportsListPosRequestFilterItemOp>,
    value: Option<ReportsListPosRequestFilterItemValue>,
}

impl ReportsListPosRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: ReportsListPosRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: ReportsListPosRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReportsListPosRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ReportsListPosRequestFilterItemBuilder::field)
    /// - [`op`](ReportsListPosRequestFilterItemBuilder::op)
    /// - [`value`](ReportsListPosRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<ReportsListPosRequestFilterItem, BuildError> {
        Ok(ReportsListPosRequestFilterItem {
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

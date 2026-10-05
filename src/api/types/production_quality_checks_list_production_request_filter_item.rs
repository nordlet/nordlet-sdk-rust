pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QualityChecksListProductionRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: QualityChecksListProductionRequestFilterItemOp,
    pub value: QualityChecksListProductionRequestFilterItemValue,
}

impl QualityChecksListProductionRequestFilterItem {
    pub fn builder() -> QualityChecksListProductionRequestFilterItemBuilder {
        <QualityChecksListProductionRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QualityChecksListProductionRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<QualityChecksListProductionRequestFilterItemOp>,
    value: Option<QualityChecksListProductionRequestFilterItemValue>,
}

impl QualityChecksListProductionRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: QualityChecksListProductionRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: QualityChecksListProductionRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QualityChecksListProductionRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](QualityChecksListProductionRequestFilterItemBuilder::field)
    /// - [`op`](QualityChecksListProductionRequestFilterItemBuilder::op)
    /// - [`value`](QualityChecksListProductionRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<QualityChecksListProductionRequestFilterItem, BuildError> {
        Ok(QualityChecksListProductionRequestFilterItem {
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

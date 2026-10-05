pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BomsListProductionRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: BomsListProductionRequestFilterItemOp,
    pub value: BomsListProductionRequestFilterItemValue,
}

impl BomsListProductionRequestFilterItem {
    pub fn builder() -> BomsListProductionRequestFilterItemBuilder {
        <BomsListProductionRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BomsListProductionRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<BomsListProductionRequestFilterItemOp>,
    value: Option<BomsListProductionRequestFilterItemValue>,
}

impl BomsListProductionRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: BomsListProductionRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: BomsListProductionRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BomsListProductionRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](BomsListProductionRequestFilterItemBuilder::field)
    /// - [`op`](BomsListProductionRequestFilterItemBuilder::op)
    /// - [`value`](BomsListProductionRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<BomsListProductionRequestFilterItem, BuildError> {
        Ok(BomsListProductionRequestFilterItem {
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

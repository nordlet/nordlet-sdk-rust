pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoutingsListProductionRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: RoutingsListProductionRequestFilterItemOp,
    pub value: RoutingsListProductionRequestFilterItemValue,
}

impl RoutingsListProductionRequestFilterItem {
    pub fn builder() -> RoutingsListProductionRequestFilterItemBuilder {
        <RoutingsListProductionRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RoutingsListProductionRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<RoutingsListProductionRequestFilterItemOp>,
    value: Option<RoutingsListProductionRequestFilterItemValue>,
}

impl RoutingsListProductionRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: RoutingsListProductionRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: RoutingsListProductionRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RoutingsListProductionRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](RoutingsListProductionRequestFilterItemBuilder::field)
    /// - [`op`](RoutingsListProductionRequestFilterItemBuilder::op)
    /// - [`value`](RoutingsListProductionRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<RoutingsListProductionRequestFilterItem, BuildError> {
        Ok(RoutingsListProductionRequestFilterItem {
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

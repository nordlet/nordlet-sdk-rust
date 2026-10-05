pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContractsListHrRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: ContractsListHrRequestFilterItemOp,
    pub value: ContractsListHrRequestFilterItemValue,
}

impl ContractsListHrRequestFilterItem {
    pub fn builder() -> ContractsListHrRequestFilterItemBuilder {
        <ContractsListHrRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContractsListHrRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<ContractsListHrRequestFilterItemOp>,
    value: Option<ContractsListHrRequestFilterItemValue>,
}

impl ContractsListHrRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: ContractsListHrRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: ContractsListHrRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ContractsListHrRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ContractsListHrRequestFilterItemBuilder::field)
    /// - [`op`](ContractsListHrRequestFilterItemBuilder::op)
    /// - [`value`](ContractsListHrRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<ContractsListHrRequestFilterItem, BuildError> {
        Ok(ContractsListHrRequestFilterItem {
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

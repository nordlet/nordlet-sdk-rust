pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DirectDebitsCandidatesBankRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: DirectDebitsCandidatesBankRequestFilterItemOp,
    pub value: DirectDebitsCandidatesBankRequestFilterItemValue,
}

impl DirectDebitsCandidatesBankRequestFilterItem {
    pub fn builder() -> DirectDebitsCandidatesBankRequestFilterItemBuilder {
        <DirectDebitsCandidatesBankRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DirectDebitsCandidatesBankRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<DirectDebitsCandidatesBankRequestFilterItemOp>,
    value: Option<DirectDebitsCandidatesBankRequestFilterItemValue>,
}

impl DirectDebitsCandidatesBankRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: DirectDebitsCandidatesBankRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: DirectDebitsCandidatesBankRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DirectDebitsCandidatesBankRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](DirectDebitsCandidatesBankRequestFilterItemBuilder::field)
    /// - [`op`](DirectDebitsCandidatesBankRequestFilterItemBuilder::op)
    /// - [`value`](DirectDebitsCandidatesBankRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<DirectDebitsCandidatesBankRequestFilterItem, BuildError> {
        Ok(DirectDebitsCandidatesBankRequestFilterItem {
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

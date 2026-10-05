pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DebtRemindersListPartnersRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: DebtRemindersListPartnersRequestFilterItemOp,
    pub value: DebtRemindersListPartnersRequestFilterItemValue,
}

impl DebtRemindersListPartnersRequestFilterItem {
    pub fn builder() -> DebtRemindersListPartnersRequestFilterItemBuilder {
        <DebtRemindersListPartnersRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DebtRemindersListPartnersRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<DebtRemindersListPartnersRequestFilterItemOp>,
    value: Option<DebtRemindersListPartnersRequestFilterItemValue>,
}

impl DebtRemindersListPartnersRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: DebtRemindersListPartnersRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: DebtRemindersListPartnersRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DebtRemindersListPartnersRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](DebtRemindersListPartnersRequestFilterItemBuilder::field)
    /// - [`op`](DebtRemindersListPartnersRequestFilterItemBuilder::op)
    /// - [`value`](DebtRemindersListPartnersRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<DebtRemindersListPartnersRequestFilterItem, BuildError> {
        Ok(DebtRemindersListPartnersRequestFilterItem {
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

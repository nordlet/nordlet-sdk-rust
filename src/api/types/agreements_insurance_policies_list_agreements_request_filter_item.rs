pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InsurancePoliciesListAgreementsRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: InsurancePoliciesListAgreementsRequestFilterItemOp,
    pub value: InsurancePoliciesListAgreementsRequestFilterItemValue,
}

impl InsurancePoliciesListAgreementsRequestFilterItem {
    pub fn builder() -> InsurancePoliciesListAgreementsRequestFilterItemBuilder {
        <InsurancePoliciesListAgreementsRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InsurancePoliciesListAgreementsRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<InsurancePoliciesListAgreementsRequestFilterItemOp>,
    value: Option<InsurancePoliciesListAgreementsRequestFilterItemValue>,
}

impl InsurancePoliciesListAgreementsRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: InsurancePoliciesListAgreementsRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: InsurancePoliciesListAgreementsRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InsurancePoliciesListAgreementsRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](InsurancePoliciesListAgreementsRequestFilterItemBuilder::field)
    /// - [`op`](InsurancePoliciesListAgreementsRequestFilterItemBuilder::op)
    /// - [`value`](InsurancePoliciesListAgreementsRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<InsurancePoliciesListAgreementsRequestFilterItem, BuildError> {
        Ok(InsurancePoliciesListAgreementsRequestFilterItem {
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

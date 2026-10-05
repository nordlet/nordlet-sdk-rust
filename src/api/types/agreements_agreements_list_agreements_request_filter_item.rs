pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgreementsListAgreementsRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: AgreementsListAgreementsRequestFilterItemOp,
    pub value: AgreementsListAgreementsRequestFilterItemValue,
}

impl AgreementsListAgreementsRequestFilterItem {
    pub fn builder() -> AgreementsListAgreementsRequestFilterItemBuilder {
        <AgreementsListAgreementsRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgreementsListAgreementsRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<AgreementsListAgreementsRequestFilterItemOp>,
    value: Option<AgreementsListAgreementsRequestFilterItemValue>,
}

impl AgreementsListAgreementsRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: AgreementsListAgreementsRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: AgreementsListAgreementsRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgreementsListAgreementsRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](AgreementsListAgreementsRequestFilterItemBuilder::field)
    /// - [`op`](AgreementsListAgreementsRequestFilterItemBuilder::op)
    /// - [`value`](AgreementsListAgreementsRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<AgreementsListAgreementsRequestFilterItem, BuildError> {
        Ok(AgreementsListAgreementsRequestFilterItem {
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

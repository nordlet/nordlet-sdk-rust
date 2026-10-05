pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TypesListAgreementsRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: TypesListAgreementsRequestFilterItemOp,
    pub value: TypesListAgreementsRequestFilterItemValue,
}

impl TypesListAgreementsRequestFilterItem {
    pub fn builder() -> TypesListAgreementsRequestFilterItemBuilder {
        <TypesListAgreementsRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TypesListAgreementsRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<TypesListAgreementsRequestFilterItemOp>,
    value: Option<TypesListAgreementsRequestFilterItemValue>,
}

impl TypesListAgreementsRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: TypesListAgreementsRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: TypesListAgreementsRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TypesListAgreementsRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](TypesListAgreementsRequestFilterItemBuilder::field)
    /// - [`op`](TypesListAgreementsRequestFilterItemBuilder::op)
    /// - [`value`](TypesListAgreementsRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<TypesListAgreementsRequestFilterItem, BuildError> {
        Ok(TypesListAgreementsRequestFilterItem {
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

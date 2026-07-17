pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1AgreementsInsurancePoliciesListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1AgreementsInsurancePoliciesListRequestFilterItemOp,
    pub value: PostV1AgreementsInsurancePoliciesListRequestFilterItemValue,
}

impl PostV1AgreementsInsurancePoliciesListRequestFilterItem {
    pub fn builder() -> PostV1AgreementsInsurancePoliciesListRequestFilterItemBuilder {
        <PostV1AgreementsInsurancePoliciesListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AgreementsInsurancePoliciesListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1AgreementsInsurancePoliciesListRequestFilterItemOp>,
    value: Option<PostV1AgreementsInsurancePoliciesListRequestFilterItemValue>,
}

impl PostV1AgreementsInsurancePoliciesListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1AgreementsInsurancePoliciesListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(
        mut self,
        value: PostV1AgreementsInsurancePoliciesListRequestFilterItemValue,
    ) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AgreementsInsurancePoliciesListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1AgreementsInsurancePoliciesListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1AgreementsInsurancePoliciesListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1AgreementsInsurancePoliciesListRequestFilterItemBuilder::value)
    pub fn build(
        self,
    ) -> Result<PostV1AgreementsInsurancePoliciesListRequestFilterItem, BuildError> {
        Ok(PostV1AgreementsInsurancePoliciesListRequestFilterItem {
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

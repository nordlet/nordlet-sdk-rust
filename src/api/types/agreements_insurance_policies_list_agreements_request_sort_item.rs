pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InsurancePoliciesListAgreementsRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<InsurancePoliciesListAgreementsRequestSortItemDir>,
}

impl InsurancePoliciesListAgreementsRequestSortItem {
    pub fn builder() -> InsurancePoliciesListAgreementsRequestSortItemBuilder {
        <InsurancePoliciesListAgreementsRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InsurancePoliciesListAgreementsRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<InsurancePoliciesListAgreementsRequestSortItemDir>,
}

impl InsurancePoliciesListAgreementsRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: InsurancePoliciesListAgreementsRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InsurancePoliciesListAgreementsRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](InsurancePoliciesListAgreementsRequestSortItemBuilder::field)
    pub fn build(self) -> Result<InsurancePoliciesListAgreementsRequestSortItem, BuildError> {
        Ok(InsurancePoliciesListAgreementsRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}

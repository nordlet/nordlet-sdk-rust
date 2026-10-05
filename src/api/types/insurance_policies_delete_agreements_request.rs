pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InsurancePoliciesDeleteAgreementsRequest {
    #[serde(default)]
    pub id: String,
}

impl InsurancePoliciesDeleteAgreementsRequest {
    pub fn builder() -> InsurancePoliciesDeleteAgreementsRequestBuilder {
        <InsurancePoliciesDeleteAgreementsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InsurancePoliciesDeleteAgreementsRequestBuilder {
    id: Option<String>,
}

impl InsurancePoliciesDeleteAgreementsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InsurancePoliciesDeleteAgreementsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InsurancePoliciesDeleteAgreementsRequestBuilder::id)
    pub fn build(self) -> Result<InsurancePoliciesDeleteAgreementsRequest, BuildError> {
        Ok(InsurancePoliciesDeleteAgreementsRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

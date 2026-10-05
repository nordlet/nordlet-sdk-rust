pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgreementsBillingRunAgreementsResponse {
    #[serde(default)]
    pub generated: Vec<AgreementsBillingRunAgreementsResponseGeneratedItem>,
    #[serde(default)]
    pub expired: Vec<String>,
    #[serde(default)]
    pub errors: Vec<AgreementsBillingRunAgreementsResponseErrorsItem>,
}

impl AgreementsBillingRunAgreementsResponse {
    pub fn builder() -> AgreementsBillingRunAgreementsResponseBuilder {
        <AgreementsBillingRunAgreementsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgreementsBillingRunAgreementsResponseBuilder {
    generated: Option<Vec<AgreementsBillingRunAgreementsResponseGeneratedItem>>,
    expired: Option<Vec<String>>,
    errors: Option<Vec<AgreementsBillingRunAgreementsResponseErrorsItem>>,
}

impl AgreementsBillingRunAgreementsResponseBuilder {
    pub fn generated(
        mut self,
        value: Vec<AgreementsBillingRunAgreementsResponseGeneratedItem>,
    ) -> Self {
        self.generated = Some(value);
        self
    }

    pub fn expired(mut self, value: Vec<String>) -> Self {
        self.expired = Some(value);
        self
    }

    pub fn errors(mut self, value: Vec<AgreementsBillingRunAgreementsResponseErrorsItem>) -> Self {
        self.errors = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgreementsBillingRunAgreementsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`generated`](AgreementsBillingRunAgreementsResponseBuilder::generated)
    /// - [`expired`](AgreementsBillingRunAgreementsResponseBuilder::expired)
    /// - [`errors`](AgreementsBillingRunAgreementsResponseBuilder::errors)
    pub fn build(self) -> Result<AgreementsBillingRunAgreementsResponse, BuildError> {
        Ok(AgreementsBillingRunAgreementsResponse {
            generated: self
                .generated
                .ok_or_else(|| BuildError::missing_field("generated"))?,
            expired: self
                .expired
                .ok_or_else(|| BuildError::missing_field("expired"))?,
            errors: self
                .errors
                .ok_or_else(|| BuildError::missing_field("errors"))?,
        })
    }
}

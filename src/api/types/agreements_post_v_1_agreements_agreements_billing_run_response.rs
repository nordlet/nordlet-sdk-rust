pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AgreementsAgreementsBillingRunResponse {
    #[serde(default)]
    pub generated: Vec<PostV1AgreementsAgreementsBillingRunResponseGeneratedItem>,
    #[serde(default)]
    pub expired: Vec<String>,
    #[serde(default)]
    pub errors: Vec<PostV1AgreementsAgreementsBillingRunResponseErrorsItem>,
}

impl PostV1AgreementsAgreementsBillingRunResponse {
    pub fn builder() -> PostV1AgreementsAgreementsBillingRunResponseBuilder {
        <PostV1AgreementsAgreementsBillingRunResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AgreementsAgreementsBillingRunResponseBuilder {
    generated: Option<Vec<PostV1AgreementsAgreementsBillingRunResponseGeneratedItem>>,
    expired: Option<Vec<String>>,
    errors: Option<Vec<PostV1AgreementsAgreementsBillingRunResponseErrorsItem>>,
}

impl PostV1AgreementsAgreementsBillingRunResponseBuilder {
    pub fn generated(
        mut self,
        value: Vec<PostV1AgreementsAgreementsBillingRunResponseGeneratedItem>,
    ) -> Self {
        self.generated = Some(value);
        self
    }

    pub fn expired(mut self, value: Vec<String>) -> Self {
        self.expired = Some(value);
        self
    }

    pub fn errors(
        mut self,
        value: Vec<PostV1AgreementsAgreementsBillingRunResponseErrorsItem>,
    ) -> Self {
        self.errors = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AgreementsAgreementsBillingRunResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`generated`](PostV1AgreementsAgreementsBillingRunResponseBuilder::generated)
    /// - [`expired`](PostV1AgreementsAgreementsBillingRunResponseBuilder::expired)
    /// - [`errors`](PostV1AgreementsAgreementsBillingRunResponseBuilder::errors)
    pub fn build(self) -> Result<PostV1AgreementsAgreementsBillingRunResponse, BuildError> {
        Ok(PostV1AgreementsAgreementsBillingRunResponse {
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

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AgreementsAgreementsBillingRunResponseErrorsItem {
    #[serde(rename = "agreementId")]
    #[serde(default)]
    pub agreement_id: String,
    #[serde(default)]
    pub message: String,
}

impl PostV1AgreementsAgreementsBillingRunResponseErrorsItem {
    pub fn builder() -> PostV1AgreementsAgreementsBillingRunResponseErrorsItemBuilder {
        <PostV1AgreementsAgreementsBillingRunResponseErrorsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AgreementsAgreementsBillingRunResponseErrorsItemBuilder {
    agreement_id: Option<String>,
    message: Option<String>,
}

impl PostV1AgreementsAgreementsBillingRunResponseErrorsItemBuilder {
    pub fn agreement_id(mut self, value: impl Into<String>) -> Self {
        self.agreement_id = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AgreementsAgreementsBillingRunResponseErrorsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agreement_id`](PostV1AgreementsAgreementsBillingRunResponseErrorsItemBuilder::agreement_id)
    /// - [`message`](PostV1AgreementsAgreementsBillingRunResponseErrorsItemBuilder::message)
    pub fn build(
        self,
    ) -> Result<PostV1AgreementsAgreementsBillingRunResponseErrorsItem, BuildError> {
        Ok(PostV1AgreementsAgreementsBillingRunResponseErrorsItem {
            agreement_id: self
                .agreement_id
                .ok_or_else(|| BuildError::missing_field("agreement_id"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}

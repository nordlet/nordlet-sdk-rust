pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgreementsBillingRunAgreementsResponseErrorsItem {
    #[serde(rename = "agreementId")]
    #[serde(default)]
    pub agreement_id: String,
    #[serde(default)]
    pub message: String,
}

impl AgreementsBillingRunAgreementsResponseErrorsItem {
    pub fn builder() -> AgreementsBillingRunAgreementsResponseErrorsItemBuilder {
        <AgreementsBillingRunAgreementsResponseErrorsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgreementsBillingRunAgreementsResponseErrorsItemBuilder {
    agreement_id: Option<String>,
    message: Option<String>,
}

impl AgreementsBillingRunAgreementsResponseErrorsItemBuilder {
    pub fn agreement_id(mut self, value: impl Into<String>) -> Self {
        self.agreement_id = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgreementsBillingRunAgreementsResponseErrorsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agreement_id`](AgreementsBillingRunAgreementsResponseErrorsItemBuilder::agreement_id)
    /// - [`message`](AgreementsBillingRunAgreementsResponseErrorsItemBuilder::message)
    pub fn build(self) -> Result<AgreementsBillingRunAgreementsResponseErrorsItem, BuildError> {
        Ok(AgreementsBillingRunAgreementsResponseErrorsItem {
            agreement_id: self
                .agreement_id
                .ok_or_else(|| BuildError::missing_field("agreement_id"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgreementsBillingRunAgreementsRequest {
    #[serde(rename = "asOfDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_of_date: Option<NaiveDate>,
}

impl AgreementsBillingRunAgreementsRequest {
    pub fn builder() -> AgreementsBillingRunAgreementsRequestBuilder {
        <AgreementsBillingRunAgreementsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgreementsBillingRunAgreementsRequestBuilder {
    as_of_date: Option<NaiveDate>,
}

impl AgreementsBillingRunAgreementsRequestBuilder {
    pub fn as_of_date(mut self, value: NaiveDate) -> Self {
        self.as_of_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgreementsBillingRunAgreementsRequest`].
    pub fn build(self) -> Result<AgreementsBillingRunAgreementsRequest, BuildError> {
        Ok(AgreementsBillingRunAgreementsRequest {
            as_of_date: self.as_of_date,
        })
    }
}

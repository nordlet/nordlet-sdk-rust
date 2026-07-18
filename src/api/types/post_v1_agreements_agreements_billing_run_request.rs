pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AgreementsAgreementsBillingRunRequest {
    #[serde(rename = "asOfDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_of_date: Option<String>,
}

impl PostV1AgreementsAgreementsBillingRunRequest {
    pub fn builder() -> PostV1AgreementsAgreementsBillingRunRequestBuilder {
        <PostV1AgreementsAgreementsBillingRunRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AgreementsAgreementsBillingRunRequestBuilder {
    as_of_date: Option<String>,
}

impl PostV1AgreementsAgreementsBillingRunRequestBuilder {
    pub fn as_of_date(mut self, value: impl Into<String>) -> Self {
        self.as_of_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AgreementsAgreementsBillingRunRequest`].
    pub fn build(self) -> Result<PostV1AgreementsAgreementsBillingRunRequest, BuildError> {
        Ok(PostV1AgreementsAgreementsBillingRunRequest {
            as_of_date: self.as_of_date,
        })
    }
}

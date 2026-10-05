pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AnnualAccountsSignaturesCreateDeclarationsResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "directorName")]
    #[serde(default)]
    pub director_name: String,
    #[serde(rename = "directorType")]
    pub director_type: AnnualAccountsSignaturesCreateDeclarationsResponseDirectorType,
    #[serde(default)]
    pub signed: bool,
    #[serde(rename = "signedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_on: Option<String>,
    #[serde(rename = "signedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub signed_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "reasonNotSigned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_not_signed: Option<String>,
}

impl AnnualAccountsSignaturesCreateDeclarationsResponse {
    pub fn builder() -> AnnualAccountsSignaturesCreateDeclarationsResponseBuilder {
        <AnnualAccountsSignaturesCreateDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnnualAccountsSignaturesCreateDeclarationsResponseBuilder {
    id: Option<String>,
    director_name: Option<String>,
    director_type: Option<AnnualAccountsSignaturesCreateDeclarationsResponseDirectorType>,
    signed: Option<bool>,
    signed_on: Option<String>,
    signed_at: Option<DateTime<FixedOffset>>,
    reason_not_signed: Option<String>,
}

impl AnnualAccountsSignaturesCreateDeclarationsResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn director_name(mut self, value: impl Into<String>) -> Self {
        self.director_name = Some(value.into());
        self
    }

    pub fn director_type(
        mut self,
        value: AnnualAccountsSignaturesCreateDeclarationsResponseDirectorType,
    ) -> Self {
        self.director_type = Some(value);
        self
    }

    pub fn signed(mut self, value: bool) -> Self {
        self.signed = Some(value);
        self
    }

    pub fn signed_on(mut self, value: impl Into<String>) -> Self {
        self.signed_on = Some(value.into());
        self
    }

    pub fn signed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.signed_at = Some(value);
        self
    }

    pub fn reason_not_signed(mut self, value: impl Into<String>) -> Self {
        self.reason_not_signed = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AnnualAccountsSignaturesCreateDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AnnualAccountsSignaturesCreateDeclarationsResponseBuilder::id)
    /// - [`director_name`](AnnualAccountsSignaturesCreateDeclarationsResponseBuilder::director_name)
    /// - [`director_type`](AnnualAccountsSignaturesCreateDeclarationsResponseBuilder::director_type)
    /// - [`signed`](AnnualAccountsSignaturesCreateDeclarationsResponseBuilder::signed)
    pub fn build(self) -> Result<AnnualAccountsSignaturesCreateDeclarationsResponse, BuildError> {
        Ok(AnnualAccountsSignaturesCreateDeclarationsResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            director_name: self
                .director_name
                .ok_or_else(|| BuildError::missing_field("director_name"))?,
            director_type: self
                .director_type
                .ok_or_else(|| BuildError::missing_field("director_type"))?,
            signed: self
                .signed
                .ok_or_else(|| BuildError::missing_field("signed"))?,
            signed_on: self.signed_on,
            signed_at: self.signed_at,
            reason_not_signed: self.reason_not_signed,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AnnualAccountsSignaturesCreateDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "directorName")]
    #[serde(default)]
    pub director_name: String,
    #[serde(rename = "directorType")]
    pub director_type: AnnualAccountsSignaturesCreateDeclarationsRequestDirectorType,
    #[serde(default)]
    pub signed: bool,
    #[serde(rename = "signedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_on: Option<NaiveDate>,
    #[serde(rename = "signedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub signed_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "reasonNotSigned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_not_signed: Option<String>,
}

impl AnnualAccountsSignaturesCreateDeclarationsRequest {
    pub fn builder() -> AnnualAccountsSignaturesCreateDeclarationsRequestBuilder {
        <AnnualAccountsSignaturesCreateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnnualAccountsSignaturesCreateDeclarationsRequestBuilder {
    year: Option<i64>,
    director_name: Option<String>,
    director_type: Option<AnnualAccountsSignaturesCreateDeclarationsRequestDirectorType>,
    signed: Option<bool>,
    signed_on: Option<NaiveDate>,
    signed_at: Option<DateTime<FixedOffset>>,
    reason_not_signed: Option<String>,
}

impl AnnualAccountsSignaturesCreateDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn director_name(mut self, value: impl Into<String>) -> Self {
        self.director_name = Some(value.into());
        self
    }

    pub fn director_type(
        mut self,
        value: AnnualAccountsSignaturesCreateDeclarationsRequestDirectorType,
    ) -> Self {
        self.director_type = Some(value);
        self
    }

    pub fn signed(mut self, value: bool) -> Self {
        self.signed = Some(value);
        self
    }

    pub fn signed_on(mut self, value: NaiveDate) -> Self {
        self.signed_on = Some(value);
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

    /// Consumes the builder and constructs a [`AnnualAccountsSignaturesCreateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](AnnualAccountsSignaturesCreateDeclarationsRequestBuilder::year)
    /// - [`director_name`](AnnualAccountsSignaturesCreateDeclarationsRequestBuilder::director_name)
    /// - [`director_type`](AnnualAccountsSignaturesCreateDeclarationsRequestBuilder::director_type)
    /// - [`signed`](AnnualAccountsSignaturesCreateDeclarationsRequestBuilder::signed)
    pub fn build(self) -> Result<AnnualAccountsSignaturesCreateDeclarationsRequest, BuildError> {
        Ok(AnnualAccountsSignaturesCreateDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
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

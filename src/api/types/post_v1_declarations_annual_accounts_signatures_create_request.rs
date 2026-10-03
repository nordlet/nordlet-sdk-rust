pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsAnnualAccountsSignaturesCreateRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "directorName")]
    #[serde(default)]
    pub director_name: String,
    #[serde(rename = "directorType")]
    pub director_type: PostV1DeclarationsAnnualAccountsSignaturesCreateRequestDirectorType,
    #[serde(default)]
    pub signed: bool,
    #[serde(rename = "signedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_on: Option<String>,
    #[serde(rename = "signedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_at: Option<String>,
    #[serde(rename = "reasonNotSigned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_not_signed: Option<String>,
}

impl PostV1DeclarationsAnnualAccountsSignaturesCreateRequest {
    pub fn builder() -> PostV1DeclarationsAnnualAccountsSignaturesCreateRequestBuilder {
        <PostV1DeclarationsAnnualAccountsSignaturesCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsAnnualAccountsSignaturesCreateRequestBuilder {
    year: Option<i64>,
    director_name: Option<String>,
    director_type: Option<PostV1DeclarationsAnnualAccountsSignaturesCreateRequestDirectorType>,
    signed: Option<bool>,
    signed_on: Option<String>,
    signed_at: Option<String>,
    reason_not_signed: Option<String>,
}

impl PostV1DeclarationsAnnualAccountsSignaturesCreateRequestBuilder {
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
        value: PostV1DeclarationsAnnualAccountsSignaturesCreateRequestDirectorType,
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

    pub fn signed_at(mut self, value: impl Into<String>) -> Self {
        self.signed_at = Some(value.into());
        self
    }

    pub fn reason_not_signed(mut self, value: impl Into<String>) -> Self {
        self.reason_not_signed = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsAnnualAccountsSignaturesCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsAnnualAccountsSignaturesCreateRequestBuilder::year)
    /// - [`director_name`](PostV1DeclarationsAnnualAccountsSignaturesCreateRequestBuilder::director_name)
    /// - [`director_type`](PostV1DeclarationsAnnualAccountsSignaturesCreateRequestBuilder::director_type)
    /// - [`signed`](PostV1DeclarationsAnnualAccountsSignaturesCreateRequestBuilder::signed)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsAnnualAccountsSignaturesCreateRequest, BuildError> {
        Ok(PostV1DeclarationsAnnualAccountsSignaturesCreateRequest {
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

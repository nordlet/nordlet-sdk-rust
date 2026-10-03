pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsAnnualAccountsSignaturesUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "directorName")]
    #[serde(default)]
    pub director_name: String,
    #[serde(rename = "directorType")]
    pub director_type: PostV1DeclarationsAnnualAccountsSignaturesUpdateRequestDirectorType,
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

impl PostV1DeclarationsAnnualAccountsSignaturesUpdateRequest {
    pub fn builder() -> PostV1DeclarationsAnnualAccountsSignaturesUpdateRequestBuilder {
        <PostV1DeclarationsAnnualAccountsSignaturesUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsAnnualAccountsSignaturesUpdateRequestBuilder {
    id: Option<String>,
    director_name: Option<String>,
    director_type: Option<PostV1DeclarationsAnnualAccountsSignaturesUpdateRequestDirectorType>,
    signed: Option<bool>,
    signed_on: Option<String>,
    signed_at: Option<String>,
    reason_not_signed: Option<String>,
}

impl PostV1DeclarationsAnnualAccountsSignaturesUpdateRequestBuilder {
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
        value: PostV1DeclarationsAnnualAccountsSignaturesUpdateRequestDirectorType,
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsAnnualAccountsSignaturesUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1DeclarationsAnnualAccountsSignaturesUpdateRequestBuilder::id)
    /// - [`director_name`](PostV1DeclarationsAnnualAccountsSignaturesUpdateRequestBuilder::director_name)
    /// - [`director_type`](PostV1DeclarationsAnnualAccountsSignaturesUpdateRequestBuilder::director_type)
    /// - [`signed`](PostV1DeclarationsAnnualAccountsSignaturesUpdateRequestBuilder::signed)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsAnnualAccountsSignaturesUpdateRequest, BuildError> {
        Ok(PostV1DeclarationsAnnualAccountsSignaturesUpdateRequest {
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

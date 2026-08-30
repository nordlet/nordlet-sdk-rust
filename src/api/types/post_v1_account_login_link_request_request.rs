pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountLoginLinkRequestRequest {
    #[serde(default)]
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<PostV1AccountLoginLinkRequestRequestLocale>,
    #[serde(rename = "acceptTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_terms: Option<bool>,
    #[serde(rename = "acceptDpa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_dpa: Option<bool>,
}

impl PostV1AccountLoginLinkRequestRequest {
    pub fn builder() -> PostV1AccountLoginLinkRequestRequestBuilder {
        <PostV1AccountLoginLinkRequestRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountLoginLinkRequestRequestBuilder {
    email: Option<String>,
    locale: Option<PostV1AccountLoginLinkRequestRequestLocale>,
    accept_terms: Option<bool>,
    accept_dpa: Option<bool>,
}

impl PostV1AccountLoginLinkRequestRequestBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn locale(mut self, value: PostV1AccountLoginLinkRequestRequestLocale) -> Self {
        self.locale = Some(value);
        self
    }

    pub fn accept_terms(mut self, value: bool) -> Self {
        self.accept_terms = Some(value);
        self
    }

    pub fn accept_dpa(mut self, value: bool) -> Self {
        self.accept_dpa = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountLoginLinkRequestRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](PostV1AccountLoginLinkRequestRequestBuilder::email)
    pub fn build(self) -> Result<PostV1AccountLoginLinkRequestRequest, BuildError> {
        Ok(PostV1AccountLoginLinkRequestRequest {
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            locale: self.locale,
            accept_terms: self.accept_terms,
            accept_dpa: self.accept_dpa,
        })
    }
}

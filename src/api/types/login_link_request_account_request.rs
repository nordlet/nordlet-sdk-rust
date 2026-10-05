pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LoginLinkRequestAccountRequest {
    #[serde(default)]
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<LoginLinkRequestAccountRequestLocale>,
    #[serde(rename = "acceptTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_terms: Option<bool>,
    #[serde(rename = "acceptDpa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_dpa: Option<bool>,
    #[serde(rename = "referralCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referral_code: Option<String>,
}

impl LoginLinkRequestAccountRequest {
    pub fn builder() -> LoginLinkRequestAccountRequestBuilder {
        <LoginLinkRequestAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LoginLinkRequestAccountRequestBuilder {
    email: Option<String>,
    locale: Option<LoginLinkRequestAccountRequestLocale>,
    accept_terms: Option<bool>,
    accept_dpa: Option<bool>,
    referral_code: Option<String>,
}

impl LoginLinkRequestAccountRequestBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn locale(mut self, value: LoginLinkRequestAccountRequestLocale) -> Self {
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

    pub fn referral_code(mut self, value: impl Into<String>) -> Self {
        self.referral_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LoginLinkRequestAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](LoginLinkRequestAccountRequestBuilder::email)
    pub fn build(self) -> Result<LoginLinkRequestAccountRequest, BuildError> {
        Ok(LoginLinkRequestAccountRequest {
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            locale: self.locale,
            accept_terms: self.accept_terms,
            accept_dpa: self.accept_dpa,
            referral_code: self.referral_code,
        })
    }
}

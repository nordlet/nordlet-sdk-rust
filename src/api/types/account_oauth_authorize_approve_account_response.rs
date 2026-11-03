pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OauthAuthorizeApproveAccountResponse {
    #[serde(rename = "redirectTo")]
    #[serde(default)]
    pub redirect_to: String,
}

impl OauthAuthorizeApproveAccountResponse {
    pub fn builder() -> OauthAuthorizeApproveAccountResponseBuilder {
        <OauthAuthorizeApproveAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OauthAuthorizeApproveAccountResponseBuilder {
    redirect_to: Option<String>,
}

impl OauthAuthorizeApproveAccountResponseBuilder {
    pub fn redirect_to(mut self, value: impl Into<String>) -> Self {
        self.redirect_to = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OauthAuthorizeApproveAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`redirect_to`](OauthAuthorizeApproveAccountResponseBuilder::redirect_to)
    pub fn build(self) -> Result<OauthAuthorizeApproveAccountResponse, BuildError> {
        Ok(OauthAuthorizeApproveAccountResponse {
            redirect_to: self
                .redirect_to
                .ok_or_else(|| BuildError::missing_field("redirect_to"))?,
        })
    }
}

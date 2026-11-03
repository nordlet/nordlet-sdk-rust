pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OauthAuthorizeDenyAccountResponse {
    #[serde(rename = "redirectTo")]
    #[serde(default)]
    pub redirect_to: String,
}

impl OauthAuthorizeDenyAccountResponse {
    pub fn builder() -> OauthAuthorizeDenyAccountResponseBuilder {
        <OauthAuthorizeDenyAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OauthAuthorizeDenyAccountResponseBuilder {
    redirect_to: Option<String>,
}

impl OauthAuthorizeDenyAccountResponseBuilder {
    pub fn redirect_to(mut self, value: impl Into<String>) -> Self {
        self.redirect_to = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OauthAuthorizeDenyAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`redirect_to`](OauthAuthorizeDenyAccountResponseBuilder::redirect_to)
    pub fn build(self) -> Result<OauthAuthorizeDenyAccountResponse, BuildError> {
        Ok(OauthAuthorizeDenyAccountResponse {
            redirect_to: self
                .redirect_to
                .ok_or_else(|| BuildError::missing_field("redirect_to"))?,
        })
    }
}

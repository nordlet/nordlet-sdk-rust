pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OauthAuthorizeDescribeAccountResponse {
    #[serde(rename = "clientName")]
    #[serde(default)]
    pub client_name: String,
    #[serde(rename = "redirectUri")]
    #[serde(default)]
    pub redirect_uri: String,
    #[serde(rename = "redirectHost")]
    #[serde(default)]
    pub redirect_host: String,
    #[serde(default)]
    pub scopes: Vec<String>,
}

impl OauthAuthorizeDescribeAccountResponse {
    pub fn builder() -> OauthAuthorizeDescribeAccountResponseBuilder {
        <OauthAuthorizeDescribeAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OauthAuthorizeDescribeAccountResponseBuilder {
    client_name: Option<String>,
    redirect_uri: Option<String>,
    redirect_host: Option<String>,
    scopes: Option<Vec<String>>,
}

impl OauthAuthorizeDescribeAccountResponseBuilder {
    pub fn client_name(mut self, value: impl Into<String>) -> Self {
        self.client_name = Some(value.into());
        self
    }

    pub fn redirect_uri(mut self, value: impl Into<String>) -> Self {
        self.redirect_uri = Some(value.into());
        self
    }

    pub fn redirect_host(mut self, value: impl Into<String>) -> Self {
        self.redirect_host = Some(value.into());
        self
    }

    pub fn scopes(mut self, value: Vec<String>) -> Self {
        self.scopes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OauthAuthorizeDescribeAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`client_name`](OauthAuthorizeDescribeAccountResponseBuilder::client_name)
    /// - [`redirect_uri`](OauthAuthorizeDescribeAccountResponseBuilder::redirect_uri)
    /// - [`redirect_host`](OauthAuthorizeDescribeAccountResponseBuilder::redirect_host)
    /// - [`scopes`](OauthAuthorizeDescribeAccountResponseBuilder::scopes)
    pub fn build(self) -> Result<OauthAuthorizeDescribeAccountResponse, BuildError> {
        Ok(OauthAuthorizeDescribeAccountResponse {
            client_name: self
                .client_name
                .ok_or_else(|| BuildError::missing_field("client_name"))?,
            redirect_uri: self
                .redirect_uri
                .ok_or_else(|| BuildError::missing_field("redirect_uri"))?,
            redirect_host: self
                .redirect_host
                .ok_or_else(|| BuildError::missing_field("redirect_host"))?,
            scopes: self
                .scopes
                .ok_or_else(|| BuildError::missing_field("scopes"))?,
        })
    }
}

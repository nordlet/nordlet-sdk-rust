pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OauthAuthorizeDenyAccountRequest {
    #[serde(rename = "clientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "redirectUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    #[serde(rename = "responseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_type: Option<String>,
    #[serde(rename = "codeChallenge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_challenge: Option<String>,
    #[serde(rename = "codeChallengeMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_challenge_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}

impl OauthAuthorizeDenyAccountRequest {
    pub fn builder() -> OauthAuthorizeDenyAccountRequestBuilder {
        <OauthAuthorizeDenyAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OauthAuthorizeDenyAccountRequestBuilder {
    client_id: Option<String>,
    redirect_uri: Option<String>,
    response_type: Option<String>,
    code_challenge: Option<String>,
    code_challenge_method: Option<String>,
    state: Option<String>,
    scope: Option<String>,
    resource: Option<String>,
}

impl OauthAuthorizeDenyAccountRequestBuilder {
    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn redirect_uri(mut self, value: impl Into<String>) -> Self {
        self.redirect_uri = Some(value.into());
        self
    }

    pub fn response_type(mut self, value: impl Into<String>) -> Self {
        self.response_type = Some(value.into());
        self
    }

    pub fn code_challenge(mut self, value: impl Into<String>) -> Self {
        self.code_challenge = Some(value.into());
        self
    }

    pub fn code_challenge_method(mut self, value: impl Into<String>) -> Self {
        self.code_challenge_method = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn scope(mut self, value: impl Into<String>) -> Self {
        self.scope = Some(value.into());
        self
    }

    pub fn resource(mut self, value: impl Into<String>) -> Self {
        self.resource = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OauthAuthorizeDenyAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`client_id`](OauthAuthorizeDenyAccountRequestBuilder::client_id)
    pub fn build(self) -> Result<OauthAuthorizeDenyAccountRequest, BuildError> {
        Ok(OauthAuthorizeDenyAccountRequest {
            client_id: self
                .client_id
                .ok_or_else(|| BuildError::missing_field("client_id"))?,
            redirect_uri: self.redirect_uri,
            response_type: self.response_type,
            code_challenge: self.code_challenge,
            code_challenge_method: self.code_challenge_method,
            state: self.state,
            scope: self.scope,
            resource: self.resource,
        })
    }
}

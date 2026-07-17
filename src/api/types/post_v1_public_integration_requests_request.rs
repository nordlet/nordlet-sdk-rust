pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PublicIntegrationRequestsRequest {
    #[serde(default)]
    pub integration: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(default)]
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}

impl PostV1PublicIntegrationRequestsRequest {
    pub fn builder() -> PostV1PublicIntegrationRequestsRequestBuilder {
        <PostV1PublicIntegrationRequestsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PublicIntegrationRequestsRequestBuilder {
    integration: Option<String>,
    name: Option<String>,
    company: Option<String>,
    email: Option<String>,
    details: Option<String>,
    website: Option<String>,
}

impl PostV1PublicIntegrationRequestsRequestBuilder {
    pub fn integration(mut self, value: impl Into<String>) -> Self {
        self.integration = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn company(mut self, value: impl Into<String>) -> Self {
        self.company = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn details(mut self, value: impl Into<String>) -> Self {
        self.details = Some(value.into());
        self
    }

    pub fn website(mut self, value: impl Into<String>) -> Self {
        self.website = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PublicIntegrationRequestsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`integration`](PostV1PublicIntegrationRequestsRequestBuilder::integration)
    /// - [`name`](PostV1PublicIntegrationRequestsRequestBuilder::name)
    /// - [`email`](PostV1PublicIntegrationRequestsRequestBuilder::email)
    pub fn build(self) -> Result<PostV1PublicIntegrationRequestsRequest, BuildError> {
        Ok(PostV1PublicIntegrationRequestsRequest {
            integration: self
                .integration
                .ok_or_else(|| BuildError::missing_field("integration"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            company: self.company,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            details: self.details,
            website: self.website,
        })
    }
}

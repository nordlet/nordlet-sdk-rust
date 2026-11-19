pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GbHmrcAuthorizeUrlGetDeclarationsResponse {
    #[serde(default)]
    pub url: String,
}

impl GbHmrcAuthorizeUrlGetDeclarationsResponse {
    pub fn builder() -> GbHmrcAuthorizeUrlGetDeclarationsResponseBuilder {
        <GbHmrcAuthorizeUrlGetDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GbHmrcAuthorizeUrlGetDeclarationsResponseBuilder {
    url: Option<String>,
}

impl GbHmrcAuthorizeUrlGetDeclarationsResponseBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GbHmrcAuthorizeUrlGetDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](GbHmrcAuthorizeUrlGetDeclarationsResponseBuilder::url)
    pub fn build(self) -> Result<GbHmrcAuthorizeUrlGetDeclarationsResponse, BuildError> {
        Ok(GbHmrcAuthorizeUrlGetDeclarationsResponse {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}

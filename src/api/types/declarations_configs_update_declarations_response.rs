pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConfigsUpdateDeclarationsResponse {
    #[serde(default)]
    pub system: String,
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub fields: Vec<ConfigsUpdateDeclarationsResponseFieldsItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<ConfigsUpdateDeclarationsResponseEndpointsItem>>,
    #[serde(default)]
    pub values: HashMap<String, String>,
    #[serde(rename = "acceptsCertificate")]
    #[serde(default)]
    pub accepts_certificate: bool,
}

impl ConfigsUpdateDeclarationsResponse {
    pub fn builder() -> ConfigsUpdateDeclarationsResponseBuilder {
        <ConfigsUpdateDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfigsUpdateDeclarationsResponseBuilder {
    system: Option<String>,
    country: Option<String>,
    title: Option<String>,
    fields: Option<Vec<ConfigsUpdateDeclarationsResponseFieldsItem>>,
    endpoints: Option<Vec<ConfigsUpdateDeclarationsResponseEndpointsItem>>,
    values: Option<HashMap<String, String>>,
    accepts_certificate: Option<bool>,
}

impl ConfigsUpdateDeclarationsResponseBuilder {
    pub fn system(mut self, value: impl Into<String>) -> Self {
        self.system = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn fields(mut self, value: Vec<ConfigsUpdateDeclarationsResponseFieldsItem>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn endpoints(mut self, value: Vec<ConfigsUpdateDeclarationsResponseEndpointsItem>) -> Self {
        self.endpoints = Some(value);
        self
    }

    pub fn values(mut self, value: HashMap<String, String>) -> Self {
        self.values = Some(value);
        self
    }

    pub fn accepts_certificate(mut self, value: bool) -> Self {
        self.accepts_certificate = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConfigsUpdateDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`system`](ConfigsUpdateDeclarationsResponseBuilder::system)
    /// - [`country`](ConfigsUpdateDeclarationsResponseBuilder::country)
    /// - [`title`](ConfigsUpdateDeclarationsResponseBuilder::title)
    /// - [`fields`](ConfigsUpdateDeclarationsResponseBuilder::fields)
    /// - [`values`](ConfigsUpdateDeclarationsResponseBuilder::values)
    /// - [`accepts_certificate`](ConfigsUpdateDeclarationsResponseBuilder::accepts_certificate)
    pub fn build(self) -> Result<ConfigsUpdateDeclarationsResponse, BuildError> {
        Ok(ConfigsUpdateDeclarationsResponse {
            system: self
                .system
                .ok_or_else(|| BuildError::missing_field("system"))?,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            fields: self
                .fields
                .ok_or_else(|| BuildError::missing_field("fields"))?,
            endpoints: self.endpoints,
            values: self
                .values
                .ok_or_else(|| BuildError::missing_field("values"))?,
            accepts_certificate: self
                .accepts_certificate
                .ok_or_else(|| BuildError::missing_field("accepts_certificate"))?,
        })
    }
}

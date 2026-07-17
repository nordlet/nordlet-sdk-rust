pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1DeclarationsConfigsUpdateResponse {
    #[serde(default)]
    pub system: String,
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub fields: Vec<PostV1DeclarationsConfigsUpdateResponseFieldsItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<PostV1DeclarationsConfigsUpdateResponseEndpointsItem>>,
    #[serde(default)]
    pub values: HashMap<String, String>,
}

impl PostV1DeclarationsConfigsUpdateResponse {
    pub fn builder() -> PostV1DeclarationsConfigsUpdateResponseBuilder {
        <PostV1DeclarationsConfigsUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsConfigsUpdateResponseBuilder {
    system: Option<String>,
    country: Option<String>,
    title: Option<String>,
    fields: Option<Vec<PostV1DeclarationsConfigsUpdateResponseFieldsItem>>,
    endpoints: Option<Vec<PostV1DeclarationsConfigsUpdateResponseEndpointsItem>>,
    values: Option<HashMap<String, String>>,
}

impl PostV1DeclarationsConfigsUpdateResponseBuilder {
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

    pub fn fields(mut self, value: Vec<PostV1DeclarationsConfigsUpdateResponseFieldsItem>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn endpoints(
        mut self,
        value: Vec<PostV1DeclarationsConfigsUpdateResponseEndpointsItem>,
    ) -> Self {
        self.endpoints = Some(value);
        self
    }

    pub fn values(mut self, value: HashMap<String, String>) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsConfigsUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`system`](PostV1DeclarationsConfigsUpdateResponseBuilder::system)
    /// - [`country`](PostV1DeclarationsConfigsUpdateResponseBuilder::country)
    /// - [`title`](PostV1DeclarationsConfigsUpdateResponseBuilder::title)
    /// - [`fields`](PostV1DeclarationsConfigsUpdateResponseBuilder::fields)
    /// - [`values`](PostV1DeclarationsConfigsUpdateResponseBuilder::values)
    pub fn build(self) -> Result<PostV1DeclarationsConfigsUpdateResponse, BuildError> {
        Ok(PostV1DeclarationsConfigsUpdateResponse {
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
        })
    }
}

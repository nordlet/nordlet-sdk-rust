pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1DeclarationsConfigsListResponseRowsItem {
    #[serde(default)]
    pub system: String,
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub fields: Vec<PostV1DeclarationsConfigsListResponseRowsItemFieldsItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<PostV1DeclarationsConfigsListResponseRowsItemEndpointsItem>>,
    #[serde(default)]
    pub values: HashMap<String, String>,
}

impl PostV1DeclarationsConfigsListResponseRowsItem {
    pub fn builder() -> PostV1DeclarationsConfigsListResponseRowsItemBuilder {
        <PostV1DeclarationsConfigsListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsConfigsListResponseRowsItemBuilder {
    system: Option<String>,
    country: Option<String>,
    title: Option<String>,
    fields: Option<Vec<PostV1DeclarationsConfigsListResponseRowsItemFieldsItem>>,
    endpoints: Option<Vec<PostV1DeclarationsConfigsListResponseRowsItemEndpointsItem>>,
    values: Option<HashMap<String, String>>,
}

impl PostV1DeclarationsConfigsListResponseRowsItemBuilder {
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

    pub fn fields(
        mut self,
        value: Vec<PostV1DeclarationsConfigsListResponseRowsItemFieldsItem>,
    ) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn endpoints(
        mut self,
        value: Vec<PostV1DeclarationsConfigsListResponseRowsItemEndpointsItem>,
    ) -> Self {
        self.endpoints = Some(value);
        self
    }

    pub fn values(mut self, value: HashMap<String, String>) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsConfigsListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`system`](PostV1DeclarationsConfigsListResponseRowsItemBuilder::system)
    /// - [`country`](PostV1DeclarationsConfigsListResponseRowsItemBuilder::country)
    /// - [`title`](PostV1DeclarationsConfigsListResponseRowsItemBuilder::title)
    /// - [`fields`](PostV1DeclarationsConfigsListResponseRowsItemBuilder::fields)
    /// - [`values`](PostV1DeclarationsConfigsListResponseRowsItemBuilder::values)
    pub fn build(self) -> Result<PostV1DeclarationsConfigsListResponseRowsItem, BuildError> {
        Ok(PostV1DeclarationsConfigsListResponseRowsItem {
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

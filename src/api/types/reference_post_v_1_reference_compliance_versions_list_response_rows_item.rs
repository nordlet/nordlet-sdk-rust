pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceComplianceVersionsListResponseRowsItem {
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub system: String,
    #[serde(default)]
    pub artifact: String,
    #[serde(default)]
    pub version: String,
    #[serde(rename = "verifiedOn")]
    #[serde(default)]
    pub verified_on: String,
    #[serde(default)]
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1ReferenceComplianceVersionsListResponseRowsItem {
    pub fn builder() -> PostV1ReferenceComplianceVersionsListResponseRowsItemBuilder {
        <PostV1ReferenceComplianceVersionsListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceComplianceVersionsListResponseRowsItemBuilder {
    country: Option<String>,
    system: Option<String>,
    artifact: Option<String>,
    version: Option<String>,
    verified_on: Option<String>,
    source: Option<String>,
    resource: Option<String>,
    notes: Option<String>,
}

impl PostV1ReferenceComplianceVersionsListResponseRowsItemBuilder {
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn system(mut self, value: impl Into<String>) -> Self {
        self.system = Some(value.into());
        self
    }

    pub fn artifact(mut self, value: impl Into<String>) -> Self {
        self.artifact = Some(value.into());
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    pub fn verified_on(mut self, value: impl Into<String>) -> Self {
        self.verified_on = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn resource(mut self, value: impl Into<String>) -> Self {
        self.resource = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceComplianceVersionsListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country`](PostV1ReferenceComplianceVersionsListResponseRowsItemBuilder::country)
    /// - [`system`](PostV1ReferenceComplianceVersionsListResponseRowsItemBuilder::system)
    /// - [`artifact`](PostV1ReferenceComplianceVersionsListResponseRowsItemBuilder::artifact)
    /// - [`version`](PostV1ReferenceComplianceVersionsListResponseRowsItemBuilder::version)
    /// - [`verified_on`](PostV1ReferenceComplianceVersionsListResponseRowsItemBuilder::verified_on)
    /// - [`source`](PostV1ReferenceComplianceVersionsListResponseRowsItemBuilder::source)
    pub fn build(
        self,
    ) -> Result<PostV1ReferenceComplianceVersionsListResponseRowsItem, BuildError> {
        Ok(PostV1ReferenceComplianceVersionsListResponseRowsItem {
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            system: self
                .system
                .ok_or_else(|| BuildError::missing_field("system"))?,
            artifact: self
                .artifact
                .ok_or_else(|| BuildError::missing_field("artifact"))?,
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
            verified_on: self
                .verified_on
                .ok_or_else(|| BuildError::missing_field("verified_on"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            resource: self.resource,
            notes: self.notes,
        })
    }
}

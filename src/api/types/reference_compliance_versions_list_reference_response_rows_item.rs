pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ComplianceVersionsListReferenceResponseRowsItem {
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

impl ComplianceVersionsListReferenceResponseRowsItem {
    pub fn builder() -> ComplianceVersionsListReferenceResponseRowsItemBuilder {
        <ComplianceVersionsListReferenceResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ComplianceVersionsListReferenceResponseRowsItemBuilder {
    country: Option<String>,
    system: Option<String>,
    artifact: Option<String>,
    version: Option<String>,
    verified_on: Option<String>,
    source: Option<String>,
    resource: Option<String>,
    notes: Option<String>,
}

impl ComplianceVersionsListReferenceResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`ComplianceVersionsListReferenceResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country`](ComplianceVersionsListReferenceResponseRowsItemBuilder::country)
    /// - [`system`](ComplianceVersionsListReferenceResponseRowsItemBuilder::system)
    /// - [`artifact`](ComplianceVersionsListReferenceResponseRowsItemBuilder::artifact)
    /// - [`version`](ComplianceVersionsListReferenceResponseRowsItemBuilder::version)
    /// - [`verified_on`](ComplianceVersionsListReferenceResponseRowsItemBuilder::verified_on)
    /// - [`source`](ComplianceVersionsListReferenceResponseRowsItemBuilder::source)
    pub fn build(self) -> Result<ComplianceVersionsListReferenceResponseRowsItem, BuildError> {
        Ok(ComplianceVersionsListReferenceResponseRowsItem {
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

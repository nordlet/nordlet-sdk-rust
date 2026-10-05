pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SourcesCreateLeadsRequest {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

impl SourcesCreateLeadsRequest {
    pub fn builder() -> SourcesCreateLeadsRequestBuilder {
        <SourcesCreateLeadsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SourcesCreateLeadsRequestBuilder {
    name: Option<String>,
    is_active: Option<bool>,
}

impl SourcesCreateLeadsRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SourcesCreateLeadsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](SourcesCreateLeadsRequestBuilder::name)
    pub fn build(self) -> Result<SourcesCreateLeadsRequest, BuildError> {
        Ok(SourcesCreateLeadsRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            is_active: self.is_active,
        })
    }
}

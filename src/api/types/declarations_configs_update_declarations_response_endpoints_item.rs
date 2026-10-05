pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConfigsUpdateDeclarationsResponseEndpointsItem {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production: Option<String>,
}

impl ConfigsUpdateDeclarationsResponseEndpointsItem {
    pub fn builder() -> ConfigsUpdateDeclarationsResponseEndpointsItemBuilder {
        <ConfigsUpdateDeclarationsResponseEndpointsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfigsUpdateDeclarationsResponseEndpointsItemBuilder {
    name: Option<String>,
    test: Option<String>,
    production: Option<String>,
}

impl ConfigsUpdateDeclarationsResponseEndpointsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn test(mut self, value: impl Into<String>) -> Self {
        self.test = Some(value.into());
        self
    }

    pub fn production(mut self, value: impl Into<String>) -> Self {
        self.production = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConfigsUpdateDeclarationsResponseEndpointsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ConfigsUpdateDeclarationsResponseEndpointsItemBuilder::name)
    pub fn build(self) -> Result<ConfigsUpdateDeclarationsResponseEndpointsItem, BuildError> {
        Ok(ConfigsUpdateDeclarationsResponseEndpointsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            test: self.test,
            production: self.production,
        })
    }
}

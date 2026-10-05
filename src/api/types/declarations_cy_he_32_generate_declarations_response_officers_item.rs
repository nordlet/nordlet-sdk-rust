pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CyHe32GenerateDeclarationsResponseOfficersItem {
    #[serde(default)]
    pub position: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

impl CyHe32GenerateDeclarationsResponseOfficersItem {
    pub fn builder() -> CyHe32GenerateDeclarationsResponseOfficersItemBuilder {
        <CyHe32GenerateDeclarationsResponseOfficersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CyHe32GenerateDeclarationsResponseOfficersItemBuilder {
    position: Option<String>,
    name: Option<String>,
    identifier: Option<String>,
}

impl CyHe32GenerateDeclarationsResponseOfficersItemBuilder {
    pub fn position(mut self, value: impl Into<String>) -> Self {
        self.position = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn identifier(mut self, value: impl Into<String>) -> Self {
        self.identifier = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CyHe32GenerateDeclarationsResponseOfficersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`position`](CyHe32GenerateDeclarationsResponseOfficersItemBuilder::position)
    /// - [`name`](CyHe32GenerateDeclarationsResponseOfficersItemBuilder::name)
    pub fn build(self) -> Result<CyHe32GenerateDeclarationsResponseOfficersItem, BuildError> {
        Ok(CyHe32GenerateDeclarationsResponseOfficersItem {
            position: self
                .position
                .ok_or_else(|| BuildError::missing_field("position"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            identifier: self.identifier,
        })
    }
}

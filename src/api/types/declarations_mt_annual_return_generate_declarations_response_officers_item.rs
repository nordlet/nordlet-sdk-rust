pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MtAnnualReturnGenerateDeclarationsResponseOfficersItem {
    #[serde(default)]
    pub position: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

impl MtAnnualReturnGenerateDeclarationsResponseOfficersItem {
    pub fn builder() -> MtAnnualReturnGenerateDeclarationsResponseOfficersItemBuilder {
        <MtAnnualReturnGenerateDeclarationsResponseOfficersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MtAnnualReturnGenerateDeclarationsResponseOfficersItemBuilder {
    position: Option<String>,
    name: Option<String>,
    identifier: Option<String>,
}

impl MtAnnualReturnGenerateDeclarationsResponseOfficersItemBuilder {
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

    /// Consumes the builder and constructs a [`MtAnnualReturnGenerateDeclarationsResponseOfficersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`position`](MtAnnualReturnGenerateDeclarationsResponseOfficersItemBuilder::position)
    /// - [`name`](MtAnnualReturnGenerateDeclarationsResponseOfficersItemBuilder::name)
    pub fn build(
        self,
    ) -> Result<MtAnnualReturnGenerateDeclarationsResponseOfficersItem, BuildError> {
        Ok(MtAnnualReturnGenerateDeclarationsResponseOfficersItem {
            position: self
                .position
                .ok_or_else(|| BuildError::missing_field("position"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            identifier: self.identifier,
        })
    }
}

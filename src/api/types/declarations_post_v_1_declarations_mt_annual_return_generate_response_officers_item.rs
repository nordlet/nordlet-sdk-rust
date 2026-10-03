pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsMtAnnualReturnGenerateResponseOfficersItem {
    #[serde(default)]
    pub position: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

impl PostV1DeclarationsMtAnnualReturnGenerateResponseOfficersItem {
    pub fn builder() -> PostV1DeclarationsMtAnnualReturnGenerateResponseOfficersItemBuilder {
        <PostV1DeclarationsMtAnnualReturnGenerateResponseOfficersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsMtAnnualReturnGenerateResponseOfficersItemBuilder {
    position: Option<String>,
    name: Option<String>,
    identifier: Option<String>,
}

impl PostV1DeclarationsMtAnnualReturnGenerateResponseOfficersItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsMtAnnualReturnGenerateResponseOfficersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`position`](PostV1DeclarationsMtAnnualReturnGenerateResponseOfficersItemBuilder::position)
    /// - [`name`](PostV1DeclarationsMtAnnualReturnGenerateResponseOfficersItemBuilder::name)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsMtAnnualReturnGenerateResponseOfficersItem, BuildError> {
        Ok(
            PostV1DeclarationsMtAnnualReturnGenerateResponseOfficersItem {
                position: self
                    .position
                    .ok_or_else(|| BuildError::missing_field("position"))?,
                name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
                identifier: self.identifier,
            },
        )
    }
}

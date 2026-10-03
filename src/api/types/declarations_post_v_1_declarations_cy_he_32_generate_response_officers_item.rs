pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsCyHe32GenerateResponseOfficersItem {
    #[serde(default)]
    pub position: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

impl PostV1DeclarationsCyHe32GenerateResponseOfficersItem {
    pub fn builder() -> PostV1DeclarationsCyHe32GenerateResponseOfficersItemBuilder {
        <PostV1DeclarationsCyHe32GenerateResponseOfficersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsCyHe32GenerateResponseOfficersItemBuilder {
    position: Option<String>,
    name: Option<String>,
    identifier: Option<String>,
}

impl PostV1DeclarationsCyHe32GenerateResponseOfficersItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsCyHe32GenerateResponseOfficersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`position`](PostV1DeclarationsCyHe32GenerateResponseOfficersItemBuilder::position)
    /// - [`name`](PostV1DeclarationsCyHe32GenerateResponseOfficersItemBuilder::name)
    pub fn build(self) -> Result<PostV1DeclarationsCyHe32GenerateResponseOfficersItem, BuildError> {
        Ok(PostV1DeclarationsCyHe32GenerateResponseOfficersItem {
            position: self
                .position
                .ok_or_else(|| BuildError::missing_field("position"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            identifier: self.identifier,
        })
    }
}

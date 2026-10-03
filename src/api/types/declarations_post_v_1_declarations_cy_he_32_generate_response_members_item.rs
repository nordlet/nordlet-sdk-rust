pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsCyHe32GenerateResponseMembersItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub identifier: String,
    #[serde(default)]
    pub shares: String,
    #[serde(rename = "nominalValue")]
    #[serde(default)]
    pub nominal_value: String,
    #[serde(rename = "shareClass")]
    #[serde(default)]
    pub share_class: String,
}

impl PostV1DeclarationsCyHe32GenerateResponseMembersItem {
    pub fn builder() -> PostV1DeclarationsCyHe32GenerateResponseMembersItemBuilder {
        <PostV1DeclarationsCyHe32GenerateResponseMembersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsCyHe32GenerateResponseMembersItemBuilder {
    name: Option<String>,
    identifier: Option<String>,
    shares: Option<String>,
    nominal_value: Option<String>,
    share_class: Option<String>,
}

impl PostV1DeclarationsCyHe32GenerateResponseMembersItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn identifier(mut self, value: impl Into<String>) -> Self {
        self.identifier = Some(value.into());
        self
    }

    pub fn shares(mut self, value: impl Into<String>) -> Self {
        self.shares = Some(value.into());
        self
    }

    pub fn nominal_value(mut self, value: impl Into<String>) -> Self {
        self.nominal_value = Some(value.into());
        self
    }

    pub fn share_class(mut self, value: impl Into<String>) -> Self {
        self.share_class = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsCyHe32GenerateResponseMembersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1DeclarationsCyHe32GenerateResponseMembersItemBuilder::name)
    /// - [`identifier`](PostV1DeclarationsCyHe32GenerateResponseMembersItemBuilder::identifier)
    /// - [`shares`](PostV1DeclarationsCyHe32GenerateResponseMembersItemBuilder::shares)
    /// - [`nominal_value`](PostV1DeclarationsCyHe32GenerateResponseMembersItemBuilder::nominal_value)
    /// - [`share_class`](PostV1DeclarationsCyHe32GenerateResponseMembersItemBuilder::share_class)
    pub fn build(self) -> Result<PostV1DeclarationsCyHe32GenerateResponseMembersItem, BuildError> {
        Ok(PostV1DeclarationsCyHe32GenerateResponseMembersItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            identifier: self
                .identifier
                .ok_or_else(|| BuildError::missing_field("identifier"))?,
            shares: self
                .shares
                .ok_or_else(|| BuildError::missing_field("shares"))?,
            nominal_value: self
                .nominal_value
                .ok_or_else(|| BuildError::missing_field("nominal_value"))?,
            share_class: self
                .share_class
                .ok_or_else(|| BuildError::missing_field("share_class"))?,
        })
    }
}

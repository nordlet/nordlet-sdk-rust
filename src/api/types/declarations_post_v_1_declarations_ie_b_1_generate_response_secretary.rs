pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsIeB1GenerateResponseSecretary {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

impl PostV1DeclarationsIeB1GenerateResponseSecretary {
    pub fn builder() -> PostV1DeclarationsIeB1GenerateResponseSecretaryBuilder {
        <PostV1DeclarationsIeB1GenerateResponseSecretaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsIeB1GenerateResponseSecretaryBuilder {
    name: Option<String>,
    identifier: Option<String>,
}

impl PostV1DeclarationsIeB1GenerateResponseSecretaryBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn identifier(mut self, value: impl Into<String>) -> Self {
        self.identifier = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsIeB1GenerateResponseSecretary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1DeclarationsIeB1GenerateResponseSecretaryBuilder::name)
    pub fn build(self) -> Result<PostV1DeclarationsIeB1GenerateResponseSecretary, BuildError> {
        Ok(PostV1DeclarationsIeB1GenerateResponseSecretary {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            identifier: self.identifier,
        })
    }
}

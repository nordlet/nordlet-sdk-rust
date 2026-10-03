pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsIeB1GenerateResponseDirectorsItem {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "appointedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appointed_on: Option<String>,
}

impl PostV1DeclarationsIeB1GenerateResponseDirectorsItem {
    pub fn builder() -> PostV1DeclarationsIeB1GenerateResponseDirectorsItemBuilder {
        <PostV1DeclarationsIeB1GenerateResponseDirectorsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsIeB1GenerateResponseDirectorsItemBuilder {
    name: Option<String>,
    identifier: Option<String>,
    appointed_on: Option<String>,
}

impl PostV1DeclarationsIeB1GenerateResponseDirectorsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn identifier(mut self, value: impl Into<String>) -> Self {
        self.identifier = Some(value.into());
        self
    }

    pub fn appointed_on(mut self, value: impl Into<String>) -> Self {
        self.appointed_on = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsIeB1GenerateResponseDirectorsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1DeclarationsIeB1GenerateResponseDirectorsItemBuilder::name)
    pub fn build(self) -> Result<PostV1DeclarationsIeB1GenerateResponseDirectorsItem, BuildError> {
        Ok(PostV1DeclarationsIeB1GenerateResponseDirectorsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            identifier: self.identifier,
            appointed_on: self.appointed_on,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LeadsCreateRequestDocumentsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#ref: String,
}

impl PostV1LeadsCreateRequestDocumentsItem {
    pub fn builder() -> PostV1LeadsCreateRequestDocumentsItemBuilder {
        <PostV1LeadsCreateRequestDocumentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LeadsCreateRequestDocumentsItemBuilder {
    name: Option<String>,
    r#ref: Option<String>,
}

impl PostV1LeadsCreateRequestDocumentsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#ref(mut self, value: impl Into<String>) -> Self {
        self.r#ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LeadsCreateRequestDocumentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1LeadsCreateRequestDocumentsItemBuilder::name)
    /// - [`r#ref`](PostV1LeadsCreateRequestDocumentsItemBuilder::r#ref)
    pub fn build(self) -> Result<PostV1LeadsCreateRequestDocumentsItem, BuildError> {
        Ok(PostV1LeadsCreateRequestDocumentsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#ref: self
                .r#ref
                .ok_or_else(|| BuildError::missing_field("r#ref"))?,
        })
    }
}

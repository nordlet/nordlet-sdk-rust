pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LeadsUpdateRequestDocumentsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#ref: String,
}

impl PostV1LeadsUpdateRequestDocumentsItem {
    pub fn builder() -> PostV1LeadsUpdateRequestDocumentsItemBuilder {
        <PostV1LeadsUpdateRequestDocumentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LeadsUpdateRequestDocumentsItemBuilder {
    name: Option<String>,
    r#ref: Option<String>,
}

impl PostV1LeadsUpdateRequestDocumentsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#ref(mut self, value: impl Into<String>) -> Self {
        self.r#ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LeadsUpdateRequestDocumentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1LeadsUpdateRequestDocumentsItemBuilder::name)
    /// - [`r#ref`](PostV1LeadsUpdateRequestDocumentsItemBuilder::r#ref)
    pub fn build(self) -> Result<PostV1LeadsUpdateRequestDocumentsItem, BuildError> {
        Ok(PostV1LeadsUpdateRequestDocumentsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#ref: self
                .r#ref
                .ok_or_else(|| BuildError::missing_field("r#ref"))?,
        })
    }
}

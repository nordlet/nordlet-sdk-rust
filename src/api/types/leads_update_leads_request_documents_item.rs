pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateLeadsRequestDocumentsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#ref: String,
}

impl UpdateLeadsRequestDocumentsItem {
    pub fn builder() -> UpdateLeadsRequestDocumentsItemBuilder {
        <UpdateLeadsRequestDocumentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateLeadsRequestDocumentsItemBuilder {
    name: Option<String>,
    r#ref: Option<String>,
}

impl UpdateLeadsRequestDocumentsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#ref(mut self, value: impl Into<String>) -> Self {
        self.r#ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateLeadsRequestDocumentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](UpdateLeadsRequestDocumentsItemBuilder::name)
    /// - [`r#ref`](UpdateLeadsRequestDocumentsItemBuilder::r#ref)
    pub fn build(self) -> Result<UpdateLeadsRequestDocumentsItem, BuildError> {
        Ok(UpdateLeadsRequestDocumentsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#ref: self
                .r#ref
                .ok_or_else(|| BuildError::missing_field("r#ref"))?,
        })
    }
}

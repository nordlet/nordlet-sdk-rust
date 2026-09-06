pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankImportTemplatesDeleteResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub deleted: bool,
}

impl PostV1BankImportTemplatesDeleteResponse {
    pub fn builder() -> PostV1BankImportTemplatesDeleteResponseBuilder {
        <PostV1BankImportTemplatesDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankImportTemplatesDeleteResponseBuilder {
    id: Option<String>,
    deleted: Option<bool>,
}

impl PostV1BankImportTemplatesDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankImportTemplatesDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1BankImportTemplatesDeleteResponseBuilder::id)
    /// - [`deleted`](PostV1BankImportTemplatesDeleteResponseBuilder::deleted)
    pub fn build(self) -> Result<PostV1BankImportTemplatesDeleteResponse, BuildError> {
        Ok(PostV1BankImportTemplatesDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}

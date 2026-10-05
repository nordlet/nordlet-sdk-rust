pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImportTemplatesDeleteBankResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub deleted: bool,
}

impl ImportTemplatesDeleteBankResponse {
    pub fn builder() -> ImportTemplatesDeleteBankResponseBuilder {
        <ImportTemplatesDeleteBankResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportTemplatesDeleteBankResponseBuilder {
    id: Option<String>,
    deleted: Option<bool>,
}

impl ImportTemplatesDeleteBankResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ImportTemplatesDeleteBankResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ImportTemplatesDeleteBankResponseBuilder::id)
    /// - [`deleted`](ImportTemplatesDeleteBankResponseBuilder::deleted)
    pub fn build(self) -> Result<ImportTemplatesDeleteBankResponse, BuildError> {
        Ok(ImportTemplatesDeleteBankResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}

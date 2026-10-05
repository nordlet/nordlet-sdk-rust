pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompaniesArchiveAccountResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub status: String,
}

impl CompaniesArchiveAccountResponse {
    pub fn builder() -> CompaniesArchiveAccountResponseBuilder {
        <CompaniesArchiveAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompaniesArchiveAccountResponseBuilder {
    id: Option<String>,
    status: Option<String>,
}

impl CompaniesArchiveAccountResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CompaniesArchiveAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CompaniesArchiveAccountResponseBuilder::id)
    /// - [`status`](CompaniesArchiveAccountResponseBuilder::status)
    pub fn build(self) -> Result<CompaniesArchiveAccountResponse, BuildError> {
        Ok(CompaniesArchiveAccountResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}

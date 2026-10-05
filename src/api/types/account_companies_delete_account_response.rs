pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompaniesDeleteAccountResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub status: String,
    #[serde(rename = "purgeAfter")]
    #[serde(default)]
    pub purge_after: String,
}

impl CompaniesDeleteAccountResponse {
    pub fn builder() -> CompaniesDeleteAccountResponseBuilder {
        <CompaniesDeleteAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompaniesDeleteAccountResponseBuilder {
    id: Option<String>,
    status: Option<String>,
    purge_after: Option<String>,
}

impl CompaniesDeleteAccountResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn purge_after(mut self, value: impl Into<String>) -> Self {
        self.purge_after = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CompaniesDeleteAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CompaniesDeleteAccountResponseBuilder::id)
    /// - [`status`](CompaniesDeleteAccountResponseBuilder::status)
    /// - [`purge_after`](CompaniesDeleteAccountResponseBuilder::purge_after)
    pub fn build(self) -> Result<CompaniesDeleteAccountResponse, BuildError> {
        Ok(CompaniesDeleteAccountResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            purge_after: self
                .purge_after
                .ok_or_else(|| BuildError::missing_field("purge_after"))?,
        })
    }
}

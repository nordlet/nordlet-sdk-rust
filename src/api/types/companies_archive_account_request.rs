pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompaniesArchiveAccountRequest {
    #[serde(rename = "companyId")]
    #[serde(default)]
    pub company_id: String,
}

impl CompaniesArchiveAccountRequest {
    pub fn builder() -> CompaniesArchiveAccountRequestBuilder {
        <CompaniesArchiveAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompaniesArchiveAccountRequestBuilder {
    company_id: Option<String>,
}

impl CompaniesArchiveAccountRequestBuilder {
    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CompaniesArchiveAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`company_id`](CompaniesArchiveAccountRequestBuilder::company_id)
    pub fn build(self) -> Result<CompaniesArchiveAccountRequest, BuildError> {
        Ok(CompaniesArchiveAccountRequest {
            company_id: self
                .company_id
                .ok_or_else(|| BuildError::missing_field("company_id"))?,
        })
    }
}

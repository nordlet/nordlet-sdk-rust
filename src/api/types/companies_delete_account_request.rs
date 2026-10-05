pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompaniesDeleteAccountRequest {
    #[serde(rename = "companyId")]
    #[serde(default)]
    pub company_id: String,
}

impl CompaniesDeleteAccountRequest {
    pub fn builder() -> CompaniesDeleteAccountRequestBuilder {
        <CompaniesDeleteAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompaniesDeleteAccountRequestBuilder {
    company_id: Option<String>,
}

impl CompaniesDeleteAccountRequestBuilder {
    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CompaniesDeleteAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`company_id`](CompaniesDeleteAccountRequestBuilder::company_id)
    pub fn build(self) -> Result<CompaniesDeleteAccountRequest, BuildError> {
        Ok(CompaniesDeleteAccountRequest {
            company_id: self
                .company_id
                .ok_or_else(|| BuildError::missing_field("company_id"))?,
        })
    }
}

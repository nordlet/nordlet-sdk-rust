pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompaniesActivateAccountRequest {
    #[serde(rename = "companyId")]
    #[serde(default)]
    pub company_id: String,
}

impl CompaniesActivateAccountRequest {
    pub fn builder() -> CompaniesActivateAccountRequestBuilder {
        <CompaniesActivateAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompaniesActivateAccountRequestBuilder {
    company_id: Option<String>,
}

impl CompaniesActivateAccountRequestBuilder {
    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CompaniesActivateAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`company_id`](CompaniesActivateAccountRequestBuilder::company_id)
    pub fn build(self) -> Result<CompaniesActivateAccountRequest, BuildError> {
        Ok(CompaniesActivateAccountRequest {
            company_id: self
                .company_id
                .ok_or_else(|| BuildError::missing_field("company_id"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompaniesSelectAccountResponse {
    #[serde(rename = "activeCompanyId")]
    #[serde(default)]
    pub active_company_id: String,
}

impl CompaniesSelectAccountResponse {
    pub fn builder() -> CompaniesSelectAccountResponseBuilder {
        <CompaniesSelectAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompaniesSelectAccountResponseBuilder {
    active_company_id: Option<String>,
}

impl CompaniesSelectAccountResponseBuilder {
    pub fn active_company_id(mut self, value: impl Into<String>) -> Self {
        self.active_company_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CompaniesSelectAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`active_company_id`](CompaniesSelectAccountResponseBuilder::active_company_id)
    pub fn build(self) -> Result<CompaniesSelectAccountResponse, BuildError> {
        Ok(CompaniesSelectAccountResponse {
            active_company_id: self
                .active_company_id
                .ok_or_else(|| BuildError::missing_field("active_company_id"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompaniesSelectAccountRequest {
    #[serde(rename = "companyId")]
    #[serde(default)]
    pub company_id: String,
}

impl CompaniesSelectAccountRequest {
    pub fn builder() -> CompaniesSelectAccountRequestBuilder {
        <CompaniesSelectAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompaniesSelectAccountRequestBuilder {
    company_id: Option<String>,
}

impl CompaniesSelectAccountRequestBuilder {
    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CompaniesSelectAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`company_id`](CompaniesSelectAccountRequestBuilder::company_id)
    pub fn build(self) -> Result<CompaniesSelectAccountRequest, BuildError> {
        Ok(CompaniesSelectAccountRequest {
            company_id: self
                .company_id
                .ok_or_else(|| BuildError::missing_field("company_id"))?,
        })
    }
}

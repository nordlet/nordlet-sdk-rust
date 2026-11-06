pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompaniesConvertToLiveAccountRequest {
    #[serde(rename = "companyId")]
    #[serde(default)]
    pub company_id: String,
}

impl CompaniesConvertToLiveAccountRequest {
    pub fn builder() -> CompaniesConvertToLiveAccountRequestBuilder {
        <CompaniesConvertToLiveAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompaniesConvertToLiveAccountRequestBuilder {
    company_id: Option<String>,
}

impl CompaniesConvertToLiveAccountRequestBuilder {
    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CompaniesConvertToLiveAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`company_id`](CompaniesConvertToLiveAccountRequestBuilder::company_id)
    pub fn build(self) -> Result<CompaniesConvertToLiveAccountRequest, BuildError> {
        Ok(CompaniesConvertToLiveAccountRequest {
            company_id: self
                .company_id
                .ok_or_else(|| BuildError::missing_field("company_id"))?,
        })
    }
}

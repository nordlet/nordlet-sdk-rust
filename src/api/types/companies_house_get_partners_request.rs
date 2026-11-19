pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompaniesHouseGetPartnersRequest {
    #[serde(rename = "companyNumber")]
    #[serde(default)]
    pub company_number: String,
}

impl CompaniesHouseGetPartnersRequest {
    pub fn builder() -> CompaniesHouseGetPartnersRequestBuilder {
        <CompaniesHouseGetPartnersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompaniesHouseGetPartnersRequestBuilder {
    company_number: Option<String>,
}

impl CompaniesHouseGetPartnersRequestBuilder {
    pub fn company_number(mut self, value: impl Into<String>) -> Self {
        self.company_number = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CompaniesHouseGetPartnersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`company_number`](CompaniesHouseGetPartnersRequestBuilder::company_number)
    pub fn build(self) -> Result<CompaniesHouseGetPartnersRequest, BuildError> {
        Ok(CompaniesHouseGetPartnersRequest {
            company_number: self
                .company_number
                .ok_or_else(|| BuildError::missing_field("company_number"))?,
        })
    }
}

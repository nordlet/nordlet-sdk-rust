pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConfigsListDeclarationsResponse {
    #[serde(rename = "companyCountry")]
    #[serde(default)]
    pub company_country: String,
    #[serde(default)]
    pub rows: Vec<ConfigsListDeclarationsResponseRowsItem>,
}

impl ConfigsListDeclarationsResponse {
    pub fn builder() -> ConfigsListDeclarationsResponseBuilder {
        <ConfigsListDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfigsListDeclarationsResponseBuilder {
    company_country: Option<String>,
    rows: Option<Vec<ConfigsListDeclarationsResponseRowsItem>>,
}

impl ConfigsListDeclarationsResponseBuilder {
    pub fn company_country(mut self, value: impl Into<String>) -> Self {
        self.company_country = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<ConfigsListDeclarationsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConfigsListDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`company_country`](ConfigsListDeclarationsResponseBuilder::company_country)
    /// - [`rows`](ConfigsListDeclarationsResponseBuilder::rows)
    pub fn build(self) -> Result<ConfigsListDeclarationsResponse, BuildError> {
        Ok(ConfigsListDeclarationsResponse {
            company_country: self
                .company_country
                .ok_or_else(|| BuildError::missing_field("company_country"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}

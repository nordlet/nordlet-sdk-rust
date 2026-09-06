pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1DeclarationsConfigsListResponse {
    #[serde(rename = "companyCountry")]
    #[serde(default)]
    pub company_country: String,
    #[serde(default)]
    pub rows: Vec<PostV1DeclarationsConfigsListResponseRowsItem>,
}

impl PostV1DeclarationsConfigsListResponse {
    pub fn builder() -> PostV1DeclarationsConfigsListResponseBuilder {
        <PostV1DeclarationsConfigsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsConfigsListResponseBuilder {
    company_country: Option<String>,
    rows: Option<Vec<PostV1DeclarationsConfigsListResponseRowsItem>>,
}

impl PostV1DeclarationsConfigsListResponseBuilder {
    pub fn company_country(mut self, value: impl Into<String>) -> Self {
        self.company_country = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<PostV1DeclarationsConfigsListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsConfigsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`company_country`](PostV1DeclarationsConfigsListResponseBuilder::company_country)
    /// - [`rows`](PostV1DeclarationsConfigsListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1DeclarationsConfigsListResponse, BuildError> {
        Ok(PostV1DeclarationsConfigsListResponse {
            company_country: self
                .company_country
                .ok_or_else(|| BuildError::missing_field("company_country"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}

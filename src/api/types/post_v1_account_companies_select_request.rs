pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountCompaniesSelectRequest {
    #[serde(rename = "companyId")]
    #[serde(default)]
    pub company_id: String,
}

impl PostV1AccountCompaniesSelectRequest {
    pub fn builder() -> PostV1AccountCompaniesSelectRequestBuilder {
        <PostV1AccountCompaniesSelectRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountCompaniesSelectRequestBuilder {
    company_id: Option<String>,
}

impl PostV1AccountCompaniesSelectRequestBuilder {
    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountCompaniesSelectRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`company_id`](PostV1AccountCompaniesSelectRequestBuilder::company_id)
    pub fn build(self) -> Result<PostV1AccountCompaniesSelectRequest, BuildError> {
        Ok(PostV1AccountCompaniesSelectRequest {
            company_id: self
                .company_id
                .ok_or_else(|| BuildError::missing_field("company_id"))?,
        })
    }
}

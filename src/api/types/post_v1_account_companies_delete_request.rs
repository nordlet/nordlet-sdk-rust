pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountCompaniesDeleteRequest {
    #[serde(rename = "companyId")]
    #[serde(default)]
    pub company_id: String,
}

impl PostV1AccountCompaniesDeleteRequest {
    pub fn builder() -> PostV1AccountCompaniesDeleteRequestBuilder {
        <PostV1AccountCompaniesDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountCompaniesDeleteRequestBuilder {
    company_id: Option<String>,
}

impl PostV1AccountCompaniesDeleteRequestBuilder {
    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountCompaniesDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`company_id`](PostV1AccountCompaniesDeleteRequestBuilder::company_id)
    pub fn build(self) -> Result<PostV1AccountCompaniesDeleteRequest, BuildError> {
        Ok(PostV1AccountCompaniesDeleteRequest {
            company_id: self
                .company_id
                .ok_or_else(|| BuildError::missing_field("company_id"))?,
        })
    }
}

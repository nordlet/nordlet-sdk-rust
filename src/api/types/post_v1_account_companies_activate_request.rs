pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountCompaniesActivateRequest {
    #[serde(rename = "companyId")]
    #[serde(default)]
    pub company_id: String,
}

impl PostV1AccountCompaniesActivateRequest {
    pub fn builder() -> PostV1AccountCompaniesActivateRequestBuilder {
        <PostV1AccountCompaniesActivateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountCompaniesActivateRequestBuilder {
    company_id: Option<String>,
}

impl PostV1AccountCompaniesActivateRequestBuilder {
    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountCompaniesActivateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`company_id`](PostV1AccountCompaniesActivateRequestBuilder::company_id)
    pub fn build(self) -> Result<PostV1AccountCompaniesActivateRequest, BuildError> {
        Ok(PostV1AccountCompaniesActivateRequest {
            company_id: self
                .company_id
                .ok_or_else(|| BuildError::missing_field("company_id"))?,
        })
    }
}

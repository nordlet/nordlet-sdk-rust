pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountCompaniesSelectResponse {
    #[serde(rename = "activeCompanyId")]
    #[serde(default)]
    pub active_company_id: String,
}

impl PostV1AccountCompaniesSelectResponse {
    pub fn builder() -> PostV1AccountCompaniesSelectResponseBuilder {
        <PostV1AccountCompaniesSelectResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountCompaniesSelectResponseBuilder {
    active_company_id: Option<String>,
}

impl PostV1AccountCompaniesSelectResponseBuilder {
    pub fn active_company_id(mut self, value: impl Into<String>) -> Self {
        self.active_company_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountCompaniesSelectResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`active_company_id`](PostV1AccountCompaniesSelectResponseBuilder::active_company_id)
    pub fn build(self) -> Result<PostV1AccountCompaniesSelectResponse, BuildError> {
        Ok(PostV1AccountCompaniesSelectResponse {
            active_company_id: self
                .active_company_id
                .ok_or_else(|| BuildError::missing_field("active_company_id"))?,
        })
    }
}

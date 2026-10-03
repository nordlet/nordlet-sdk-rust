pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsMtCompanyTaxGenerateRequest {
    #[serde(default)]
    pub year: i64,
}

impl PostV1DeclarationsMtCompanyTaxGenerateRequest {
    pub fn builder() -> PostV1DeclarationsMtCompanyTaxGenerateRequestBuilder {
        <PostV1DeclarationsMtCompanyTaxGenerateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsMtCompanyTaxGenerateRequestBuilder {
    year: Option<i64>,
}

impl PostV1DeclarationsMtCompanyTaxGenerateRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsMtCompanyTaxGenerateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsMtCompanyTaxGenerateRequestBuilder::year)
    pub fn build(self) -> Result<PostV1DeclarationsMtCompanyTaxGenerateRequest, BuildError> {
        Ok(PostV1DeclarationsMtCompanyTaxGenerateRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}

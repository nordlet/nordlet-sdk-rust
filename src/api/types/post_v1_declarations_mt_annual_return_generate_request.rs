pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsMtAnnualReturnGenerateRequest {
    #[serde(default)]
    pub year: i64,
}

impl PostV1DeclarationsMtAnnualReturnGenerateRequest {
    pub fn builder() -> PostV1DeclarationsMtAnnualReturnGenerateRequestBuilder {
        <PostV1DeclarationsMtAnnualReturnGenerateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsMtAnnualReturnGenerateRequestBuilder {
    year: Option<i64>,
}

impl PostV1DeclarationsMtAnnualReturnGenerateRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsMtAnnualReturnGenerateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsMtAnnualReturnGenerateRequestBuilder::year)
    pub fn build(self) -> Result<PostV1DeclarationsMtAnnualReturnGenerateRequest, BuildError> {
        Ok(PostV1DeclarationsMtAnnualReturnGenerateRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}

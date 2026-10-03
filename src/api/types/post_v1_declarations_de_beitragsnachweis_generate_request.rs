pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsDeBeitragsnachweisGenerateRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
}

impl PostV1DeclarationsDeBeitragsnachweisGenerateRequest {
    pub fn builder() -> PostV1DeclarationsDeBeitragsnachweisGenerateRequestBuilder {
        <PostV1DeclarationsDeBeitragsnachweisGenerateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsDeBeitragsnachweisGenerateRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
}

impl PostV1DeclarationsDeBeitragsnachweisGenerateRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsDeBeitragsnachweisGenerateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsDeBeitragsnachweisGenerateRequestBuilder::year)
    /// - [`month`](PostV1DeclarationsDeBeitragsnachweisGenerateRequestBuilder::month)
    pub fn build(self) -> Result<PostV1DeclarationsDeBeitragsnachweisGenerateRequest, BuildError> {
        Ok(PostV1DeclarationsDeBeitragsnachweisGenerateRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
        })
    }
}

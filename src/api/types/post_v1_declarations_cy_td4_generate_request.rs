pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsCyTd4GenerateRequest {
    #[serde(default)]
    pub year: i64,
}

impl PostV1DeclarationsCyTd4GenerateRequest {
    pub fn builder() -> PostV1DeclarationsCyTd4GenerateRequestBuilder {
        <PostV1DeclarationsCyTd4GenerateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsCyTd4GenerateRequestBuilder {
    year: Option<i64>,
}

impl PostV1DeclarationsCyTd4GenerateRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsCyTd4GenerateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsCyTd4GenerateRequestBuilder::year)
    pub fn build(self) -> Result<PostV1DeclarationsCyTd4GenerateRequest, BuildError> {
        Ok(PostV1DeclarationsCyTd4GenerateRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}

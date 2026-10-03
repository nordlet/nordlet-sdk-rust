pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsCyHe32GenerateRequest {
    #[serde(default)]
    pub year: i64,
}

impl PostV1DeclarationsCyHe32GenerateRequest {
    pub fn builder() -> PostV1DeclarationsCyHe32GenerateRequestBuilder {
        <PostV1DeclarationsCyHe32GenerateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsCyHe32GenerateRequestBuilder {
    year: Option<i64>,
}

impl PostV1DeclarationsCyHe32GenerateRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsCyHe32GenerateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsCyHe32GenerateRequestBuilder::year)
    pub fn build(self) -> Result<PostV1DeclarationsCyHe32GenerateRequest, BuildError> {
        Ok(PostV1DeclarationsCyHe32GenerateRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}

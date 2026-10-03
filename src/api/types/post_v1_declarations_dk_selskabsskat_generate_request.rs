pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsDkSelskabsskatGenerateRequest {
    #[serde(default)]
    pub year: i64,
}

impl PostV1DeclarationsDkSelskabsskatGenerateRequest {
    pub fn builder() -> PostV1DeclarationsDkSelskabsskatGenerateRequestBuilder {
        <PostV1DeclarationsDkSelskabsskatGenerateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsDkSelskabsskatGenerateRequestBuilder {
    year: Option<i64>,
}

impl PostV1DeclarationsDkSelskabsskatGenerateRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsDkSelskabsskatGenerateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsDkSelskabsskatGenerateRequestBuilder::year)
    pub fn build(self) -> Result<PostV1DeclarationsDkSelskabsskatGenerateRequest, BuildError> {
        Ok(PostV1DeclarationsDkSelskabsskatGenerateRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}

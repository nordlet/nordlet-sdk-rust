pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsTaxAdjustmentsListRequest {
    #[serde(default)]
    pub year: i64,
}

impl PostV1DeclarationsTaxAdjustmentsListRequest {
    pub fn builder() -> PostV1DeclarationsTaxAdjustmentsListRequestBuilder {
        <PostV1DeclarationsTaxAdjustmentsListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsTaxAdjustmentsListRequestBuilder {
    year: Option<i64>,
}

impl PostV1DeclarationsTaxAdjustmentsListRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsTaxAdjustmentsListRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsTaxAdjustmentsListRequestBuilder::year)
    pub fn build(self) -> Result<PostV1DeclarationsTaxAdjustmentsListRequest, BuildError> {
        Ok(PostV1DeclarationsTaxAdjustmentsListRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}

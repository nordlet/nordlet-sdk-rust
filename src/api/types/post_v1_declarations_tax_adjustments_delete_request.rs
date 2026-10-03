pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsTaxAdjustmentsDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1DeclarationsTaxAdjustmentsDeleteRequest {
    pub fn builder() -> PostV1DeclarationsTaxAdjustmentsDeleteRequestBuilder {
        <PostV1DeclarationsTaxAdjustmentsDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsTaxAdjustmentsDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1DeclarationsTaxAdjustmentsDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsTaxAdjustmentsDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1DeclarationsTaxAdjustmentsDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1DeclarationsTaxAdjustmentsDeleteRequest, BuildError> {
        Ok(PostV1DeclarationsTaxAdjustmentsDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

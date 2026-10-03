pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsTaxAdjustmentsDeleteResponse {
    #[serde(default)]
    pub id: String,
}

impl PostV1DeclarationsTaxAdjustmentsDeleteResponse {
    pub fn builder() -> PostV1DeclarationsTaxAdjustmentsDeleteResponseBuilder {
        <PostV1DeclarationsTaxAdjustmentsDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsTaxAdjustmentsDeleteResponseBuilder {
    id: Option<String>,
}

impl PostV1DeclarationsTaxAdjustmentsDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsTaxAdjustmentsDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1DeclarationsTaxAdjustmentsDeleteResponseBuilder::id)
    pub fn build(self) -> Result<PostV1DeclarationsTaxAdjustmentsDeleteResponse, BuildError> {
        Ok(PostV1DeclarationsTaxAdjustmentsDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

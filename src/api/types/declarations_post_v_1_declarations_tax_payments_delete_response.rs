pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsTaxPaymentsDeleteResponse {
    #[serde(default)]
    pub id: String,
}

impl PostV1DeclarationsTaxPaymentsDeleteResponse {
    pub fn builder() -> PostV1DeclarationsTaxPaymentsDeleteResponseBuilder {
        <PostV1DeclarationsTaxPaymentsDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsTaxPaymentsDeleteResponseBuilder {
    id: Option<String>,
}

impl PostV1DeclarationsTaxPaymentsDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsTaxPaymentsDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1DeclarationsTaxPaymentsDeleteResponseBuilder::id)
    pub fn build(self) -> Result<PostV1DeclarationsTaxPaymentsDeleteResponse, BuildError> {
        Ok(PostV1DeclarationsTaxPaymentsDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

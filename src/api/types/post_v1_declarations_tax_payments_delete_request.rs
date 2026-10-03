pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsTaxPaymentsDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1DeclarationsTaxPaymentsDeleteRequest {
    pub fn builder() -> PostV1DeclarationsTaxPaymentsDeleteRequestBuilder {
        <PostV1DeclarationsTaxPaymentsDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsTaxPaymentsDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1DeclarationsTaxPaymentsDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsTaxPaymentsDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1DeclarationsTaxPaymentsDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1DeclarationsTaxPaymentsDeleteRequest, BuildError> {
        Ok(PostV1DeclarationsTaxPaymentsDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

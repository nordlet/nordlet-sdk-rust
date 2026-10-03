pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsCertificatesDeleteRequest {
    #[serde(default)]
    pub system: String,
    #[serde(rename = "fieldKey")]
    pub field_key: PostV1DeclarationsCertificatesDeleteRequestFieldKey,
}

impl PostV1DeclarationsCertificatesDeleteRequest {
    pub fn builder() -> PostV1DeclarationsCertificatesDeleteRequestBuilder {
        <PostV1DeclarationsCertificatesDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsCertificatesDeleteRequestBuilder {
    system: Option<String>,
    field_key: Option<PostV1DeclarationsCertificatesDeleteRequestFieldKey>,
}

impl PostV1DeclarationsCertificatesDeleteRequestBuilder {
    pub fn system(mut self, value: impl Into<String>) -> Self {
        self.system = Some(value.into());
        self
    }

    pub fn field_key(mut self, value: PostV1DeclarationsCertificatesDeleteRequestFieldKey) -> Self {
        self.field_key = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsCertificatesDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`system`](PostV1DeclarationsCertificatesDeleteRequestBuilder::system)
    /// - [`field_key`](PostV1DeclarationsCertificatesDeleteRequestBuilder::field_key)
    pub fn build(self) -> Result<PostV1DeclarationsCertificatesDeleteRequest, BuildError> {
        Ok(PostV1DeclarationsCertificatesDeleteRequest {
            system: self
                .system
                .ok_or_else(|| BuildError::missing_field("system"))?,
            field_key: self
                .field_key
                .ok_or_else(|| BuildError::missing_field("field_key"))?,
        })
    }
}

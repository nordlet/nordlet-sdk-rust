pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlKsefReceiptRequest {
    #[serde(rename = "sessionReferenceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_reference_number: Option<String>,
}

impl PostV1DeclarationsPlKsefReceiptRequest {
    pub fn builder() -> PostV1DeclarationsPlKsefReceiptRequestBuilder {
        <PostV1DeclarationsPlKsefReceiptRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlKsefReceiptRequestBuilder {
    session_reference_number: Option<String>,
}

impl PostV1DeclarationsPlKsefReceiptRequestBuilder {
    pub fn session_reference_number(mut self, value: impl Into<String>) -> Self {
        self.session_reference_number = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlKsefReceiptRequest`].
    pub fn build(self) -> Result<PostV1DeclarationsPlKsefReceiptRequest, BuildError> {
        Ok(PostV1DeclarationsPlKsefReceiptRequest {
            session_reference_number: self.session_reference_number,
        })
    }
}

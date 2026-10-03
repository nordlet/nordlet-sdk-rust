pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsRoEtransportStatusRequest {
    #[serde(default)]
    pub reference: String,
}

impl PostV1DeclarationsRoEtransportStatusRequest {
    pub fn builder() -> PostV1DeclarationsRoEtransportStatusRequestBuilder {
        <PostV1DeclarationsRoEtransportStatusRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsRoEtransportStatusRequestBuilder {
    reference: Option<String>,
}

impl PostV1DeclarationsRoEtransportStatusRequestBuilder {
    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsRoEtransportStatusRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reference`](PostV1DeclarationsRoEtransportStatusRequestBuilder::reference)
    pub fn build(self) -> Result<PostV1DeclarationsRoEtransportStatusRequest, BuildError> {
        Ok(PostV1DeclarationsRoEtransportStatusRequest {
            reference: self
                .reference
                .ok_or_else(|| BuildError::missing_field("reference"))?,
        })
    }
}

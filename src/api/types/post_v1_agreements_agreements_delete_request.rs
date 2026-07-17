pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AgreementsAgreementsDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1AgreementsAgreementsDeleteRequest {
    pub fn builder() -> PostV1AgreementsAgreementsDeleteRequestBuilder {
        <PostV1AgreementsAgreementsDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AgreementsAgreementsDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1AgreementsAgreementsDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AgreementsAgreementsDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AgreementsAgreementsDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1AgreementsAgreementsDeleteRequest, BuildError> {
        Ok(PostV1AgreementsAgreementsDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

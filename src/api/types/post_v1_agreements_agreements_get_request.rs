pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AgreementsAgreementsGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1AgreementsAgreementsGetRequest {
    pub fn builder() -> PostV1AgreementsAgreementsGetRequestBuilder {
        <PostV1AgreementsAgreementsGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AgreementsAgreementsGetRequestBuilder {
    id: Option<String>,
}

impl PostV1AgreementsAgreementsGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AgreementsAgreementsGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AgreementsAgreementsGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1AgreementsAgreementsGetRequest, BuildError> {
        Ok(PostV1AgreementsAgreementsGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

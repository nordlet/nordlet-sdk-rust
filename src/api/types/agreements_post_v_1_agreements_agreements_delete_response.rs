pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AgreementsAgreementsDeleteResponse {
    #[serde(default)]
    pub id: String,
}

impl PostV1AgreementsAgreementsDeleteResponse {
    pub fn builder() -> PostV1AgreementsAgreementsDeleteResponseBuilder {
        <PostV1AgreementsAgreementsDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AgreementsAgreementsDeleteResponseBuilder {
    id: Option<String>,
}

impl PostV1AgreementsAgreementsDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AgreementsAgreementsDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AgreementsAgreementsDeleteResponseBuilder::id)
    pub fn build(self) -> Result<PostV1AgreementsAgreementsDeleteResponse, BuildError> {
        Ok(PostV1AgreementsAgreementsDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

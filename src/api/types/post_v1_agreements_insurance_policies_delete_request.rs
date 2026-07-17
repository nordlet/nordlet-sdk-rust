pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AgreementsInsurancePoliciesDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1AgreementsInsurancePoliciesDeleteRequest {
    pub fn builder() -> PostV1AgreementsInsurancePoliciesDeleteRequestBuilder {
        <PostV1AgreementsInsurancePoliciesDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AgreementsInsurancePoliciesDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1AgreementsInsurancePoliciesDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AgreementsInsurancePoliciesDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AgreementsInsurancePoliciesDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1AgreementsInsurancePoliciesDeleteRequest, BuildError> {
        Ok(PostV1AgreementsInsurancePoliciesDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

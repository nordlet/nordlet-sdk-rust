pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AgreementsInsurancePoliciesDeleteResponse {
    #[serde(default)]
    pub id: String,
}

impl PostV1AgreementsInsurancePoliciesDeleteResponse {
    pub fn builder() -> PostV1AgreementsInsurancePoliciesDeleteResponseBuilder {
        <PostV1AgreementsInsurancePoliciesDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AgreementsInsurancePoliciesDeleteResponseBuilder {
    id: Option<String>,
}

impl PostV1AgreementsInsurancePoliciesDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AgreementsInsurancePoliciesDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AgreementsInsurancePoliciesDeleteResponseBuilder::id)
    pub fn build(self) -> Result<PostV1AgreementsInsurancePoliciesDeleteResponse, BuildError> {
        Ok(PostV1AgreementsInsurancePoliciesDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

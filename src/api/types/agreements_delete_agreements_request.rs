pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgreementsDeleteAgreementsRequest {
    #[serde(default)]
    pub id: String,
}

impl AgreementsDeleteAgreementsRequest {
    pub fn builder() -> AgreementsDeleteAgreementsRequestBuilder {
        <AgreementsDeleteAgreementsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgreementsDeleteAgreementsRequestBuilder {
    id: Option<String>,
}

impl AgreementsDeleteAgreementsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgreementsDeleteAgreementsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AgreementsDeleteAgreementsRequestBuilder::id)
    pub fn build(self) -> Result<AgreementsDeleteAgreementsRequest, BuildError> {
        Ok(AgreementsDeleteAgreementsRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

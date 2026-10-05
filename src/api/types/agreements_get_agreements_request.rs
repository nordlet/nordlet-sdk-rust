pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgreementsGetAgreementsRequest {
    #[serde(default)]
    pub id: String,
}

impl AgreementsGetAgreementsRequest {
    pub fn builder() -> AgreementsGetAgreementsRequestBuilder {
        <AgreementsGetAgreementsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgreementsGetAgreementsRequestBuilder {
    id: Option<String>,
}

impl AgreementsGetAgreementsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgreementsGetAgreementsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AgreementsGetAgreementsRequestBuilder::id)
    pub fn build(self) -> Result<AgreementsGetAgreementsRequest, BuildError> {
        Ok(AgreementsGetAgreementsRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

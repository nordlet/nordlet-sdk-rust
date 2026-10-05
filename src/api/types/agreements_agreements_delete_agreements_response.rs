pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgreementsDeleteAgreementsResponse {
    #[serde(default)]
    pub id: String,
}

impl AgreementsDeleteAgreementsResponse {
    pub fn builder() -> AgreementsDeleteAgreementsResponseBuilder {
        <AgreementsDeleteAgreementsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgreementsDeleteAgreementsResponseBuilder {
    id: Option<String>,
}

impl AgreementsDeleteAgreementsResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgreementsDeleteAgreementsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AgreementsDeleteAgreementsResponseBuilder::id)
    pub fn build(self) -> Result<AgreementsDeleteAgreementsResponse, BuildError> {
        Ok(AgreementsDeleteAgreementsResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

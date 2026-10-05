pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AnonymizePartnersResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub anonymized: bool,
}

impl AnonymizePartnersResponse {
    pub fn builder() -> AnonymizePartnersResponseBuilder {
        <AnonymizePartnersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnonymizePartnersResponseBuilder {
    id: Option<String>,
    anonymized: Option<bool>,
}

impl AnonymizePartnersResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn anonymized(mut self, value: bool) -> Self {
        self.anonymized = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AnonymizePartnersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AnonymizePartnersResponseBuilder::id)
    /// - [`anonymized`](AnonymizePartnersResponseBuilder::anonymized)
    pub fn build(self) -> Result<AnonymizePartnersResponse, BuildError> {
        Ok(AnonymizePartnersResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            anonymized: self
                .anonymized
                .ok_or_else(|| BuildError::missing_field("anonymized"))?,
        })
    }
}

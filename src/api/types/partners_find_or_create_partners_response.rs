pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FindOrCreatePartnersResponse {
    #[serde(default)]
    pub created: bool,
    pub partner: FindOrCreatePartnersResponsePartner,
}

impl FindOrCreatePartnersResponse {
    pub fn builder() -> FindOrCreatePartnersResponseBuilder {
        <FindOrCreatePartnersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FindOrCreatePartnersResponseBuilder {
    created: Option<bool>,
    partner: Option<FindOrCreatePartnersResponsePartner>,
}

impl FindOrCreatePartnersResponseBuilder {
    pub fn created(mut self, value: bool) -> Self {
        self.created = Some(value);
        self
    }

    pub fn partner(mut self, value: FindOrCreatePartnersResponsePartner) -> Self {
        self.partner = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FindOrCreatePartnersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created`](FindOrCreatePartnersResponseBuilder::created)
    /// - [`partner`](FindOrCreatePartnersResponseBuilder::partner)
    pub fn build(self) -> Result<FindOrCreatePartnersResponse, BuildError> {
        Ok(FindOrCreatePartnersResponse {
            created: self
                .created
                .ok_or_else(|| BuildError::missing_field("created"))?,
            partner: self
                .partner
                .ok_or_else(|| BuildError::missing_field("partner"))?,
        })
    }
}

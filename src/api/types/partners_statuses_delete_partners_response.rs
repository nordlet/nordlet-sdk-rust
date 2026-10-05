pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StatusesDeletePartnersResponse {
    #[serde(default)]
    pub id: String,
}

impl StatusesDeletePartnersResponse {
    pub fn builder() -> StatusesDeletePartnersResponseBuilder {
        <StatusesDeletePartnersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatusesDeletePartnersResponseBuilder {
    id: Option<String>,
}

impl StatusesDeletePartnersResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StatusesDeletePartnersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](StatusesDeletePartnersResponseBuilder::id)
    pub fn build(self) -> Result<StatusesDeletePartnersResponse, BuildError> {
        Ok(StatusesDeletePartnersResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

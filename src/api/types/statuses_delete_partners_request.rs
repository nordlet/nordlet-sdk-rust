pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StatusesDeletePartnersRequest {
    #[serde(default)]
    pub id: String,
}

impl StatusesDeletePartnersRequest {
    pub fn builder() -> StatusesDeletePartnersRequestBuilder {
        <StatusesDeletePartnersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatusesDeletePartnersRequestBuilder {
    id: Option<String>,
}

impl StatusesDeletePartnersRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StatusesDeletePartnersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](StatusesDeletePartnersRequestBuilder::id)
    pub fn build(self) -> Result<StatusesDeletePartnersRequest, BuildError> {
        Ok(StatusesDeletePartnersRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

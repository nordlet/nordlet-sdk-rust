pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetPartnersRequest {
    #[serde(default)]
    pub id: String,
}

impl GetPartnersRequest {
    pub fn builder() -> GetPartnersRequestBuilder {
        <GetPartnersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPartnersRequestBuilder {
    id: Option<String>,
}

impl GetPartnersRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetPartnersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetPartnersRequestBuilder::id)
    pub fn build(self) -> Result<GetPartnersRequest, BuildError> {
        Ok(GetPartnersRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

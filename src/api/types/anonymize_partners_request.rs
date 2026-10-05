pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AnonymizePartnersRequest {
    #[serde(default)]
    pub id: String,
}

impl AnonymizePartnersRequest {
    pub fn builder() -> AnonymizePartnersRequestBuilder {
        <AnonymizePartnersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnonymizePartnersRequestBuilder {
    id: Option<String>,
}

impl AnonymizePartnersRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AnonymizePartnersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AnonymizePartnersRequestBuilder::id)
    pub fn build(self) -> Result<AnonymizePartnersRequest, BuildError> {
        Ok(AnonymizePartnersRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

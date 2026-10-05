pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeletePartnersRequest {
    #[serde(default)]
    pub id: String,
}

impl DeletePartnersRequest {
    pub fn builder() -> DeletePartnersRequestBuilder {
        <DeletePartnersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeletePartnersRequestBuilder {
    id: Option<String>,
}

impl DeletePartnersRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeletePartnersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DeletePartnersRequestBuilder::id)
    pub fn build(self) -> Result<DeletePartnersRequest, BuildError> {
        Ok(DeletePartnersRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

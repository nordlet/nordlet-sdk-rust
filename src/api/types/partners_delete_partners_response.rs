pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeletePartnersResponse {
    #[serde(default)]
    pub id: String,
}

impl DeletePartnersResponse {
    pub fn builder() -> DeletePartnersResponseBuilder {
        <DeletePartnersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeletePartnersResponseBuilder {
    id: Option<String>,
}

impl DeletePartnersResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeletePartnersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DeletePartnersResponseBuilder::id)
    pub fn build(self) -> Result<DeletePartnersResponse, BuildError> {
        Ok(DeletePartnersResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

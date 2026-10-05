pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteLeadsResponse {
    #[serde(default)]
    pub id: String,
}

impl DeleteLeadsResponse {
    pub fn builder() -> DeleteLeadsResponseBuilder {
        <DeleteLeadsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteLeadsResponseBuilder {
    id: Option<String>,
}

impl DeleteLeadsResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteLeadsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DeleteLeadsResponseBuilder::id)
    pub fn build(self) -> Result<DeleteLeadsResponse, BuildError> {
        Ok(DeleteLeadsResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

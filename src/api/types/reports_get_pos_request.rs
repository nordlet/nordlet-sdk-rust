pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportsGetPosRequest {
    #[serde(default)]
    pub id: String,
}

impl ReportsGetPosRequest {
    pub fn builder() -> ReportsGetPosRequestBuilder {
        <ReportsGetPosRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportsGetPosRequestBuilder {
    id: Option<String>,
}

impl ReportsGetPosRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReportsGetPosRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReportsGetPosRequestBuilder::id)
    pub fn build(self) -> Result<ReportsGetPosRequest, BuildError> {
        Ok(ReportsGetPosRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

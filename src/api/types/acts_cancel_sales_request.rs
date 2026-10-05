pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ActsCancelSalesRequest {
    #[serde(default)]
    pub id: String,
}

impl ActsCancelSalesRequest {
    pub fn builder() -> ActsCancelSalesRequestBuilder {
        <ActsCancelSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ActsCancelSalesRequestBuilder {
    id: Option<String>,
}

impl ActsCancelSalesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ActsCancelSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ActsCancelSalesRequestBuilder::id)
    pub fn build(self) -> Result<ActsCancelSalesRequest, BuildError> {
        Ok(ActsCancelSalesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

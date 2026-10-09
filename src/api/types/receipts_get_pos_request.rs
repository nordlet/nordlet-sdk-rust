pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceiptsGetPosRequest {
    #[serde(default)]
    pub id: String,
}

impl ReceiptsGetPosRequest {
    pub fn builder() -> ReceiptsGetPosRequestBuilder {
        <ReceiptsGetPosRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceiptsGetPosRequestBuilder {
    id: Option<String>,
}

impl ReceiptsGetPosRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReceiptsGetPosRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReceiptsGetPosRequestBuilder::id)
    pub fn build(self) -> Result<ReceiptsGetPosRequest, BuildError> {
        Ok(ReceiptsGetPosRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

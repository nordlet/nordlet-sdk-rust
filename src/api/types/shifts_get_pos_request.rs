pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ShiftsGetPosRequest {
    #[serde(default)]
    pub id: String,
}

impl ShiftsGetPosRequest {
    pub fn builder() -> ShiftsGetPosRequestBuilder {
        <ShiftsGetPosRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ShiftsGetPosRequestBuilder {
    id: Option<String>,
}

impl ShiftsGetPosRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ShiftsGetPosRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ShiftsGetPosRequestBuilder::id)
    pub fn build(self) -> Result<ShiftsGetPosRequest, BuildError> {
        Ok(ShiftsGetPosRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

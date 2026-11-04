pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RefundsGetPosRequest {
    #[serde(default)]
    pub id: String,
}

impl RefundsGetPosRequest {
    pub fn builder() -> RefundsGetPosRequestBuilder {
        <RefundsGetPosRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefundsGetPosRequestBuilder {
    id: Option<String>,
}

impl RefundsGetPosRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RefundsGetPosRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RefundsGetPosRequestBuilder::id)
    pub fn build(self) -> Result<RefundsGetPosRequest, BuildError> {
        Ok(RefundsGetPosRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

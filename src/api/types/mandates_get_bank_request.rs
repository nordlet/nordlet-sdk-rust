pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MandatesGetBankRequest {
    #[serde(default)]
    pub id: String,
}

impl MandatesGetBankRequest {
    pub fn builder() -> MandatesGetBankRequestBuilder {
        <MandatesGetBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MandatesGetBankRequestBuilder {
    id: Option<String>,
}

impl MandatesGetBankRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MandatesGetBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MandatesGetBankRequestBuilder::id)
    pub fn build(self) -> Result<MandatesGetBankRequest, BuildError> {
        Ok(MandatesGetBankRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SecuritiesGetBankRequest {
    #[serde(default)]
    pub id: String,
}

impl SecuritiesGetBankRequest {
    pub fn builder() -> SecuritiesGetBankRequestBuilder {
        <SecuritiesGetBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SecuritiesGetBankRequestBuilder {
    id: Option<String>,
}

impl SecuritiesGetBankRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SecuritiesGetBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SecuritiesGetBankRequestBuilder::id)
    pub fn build(self) -> Result<SecuritiesGetBankRequest, BuildError> {
        Ok(SecuritiesGetBankRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

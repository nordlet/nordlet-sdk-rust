pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExchangeRatesOverridesDeleteReferenceResponse {
    #[serde(default)]
    pub deleted: bool,
}

impl ExchangeRatesOverridesDeleteReferenceResponse {
    pub fn builder() -> ExchangeRatesOverridesDeleteReferenceResponseBuilder {
        <ExchangeRatesOverridesDeleteReferenceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExchangeRatesOverridesDeleteReferenceResponseBuilder {
    deleted: Option<bool>,
}

impl ExchangeRatesOverridesDeleteReferenceResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExchangeRatesOverridesDeleteReferenceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](ExchangeRatesOverridesDeleteReferenceResponseBuilder::deleted)
    pub fn build(self) -> Result<ExchangeRatesOverridesDeleteReferenceResponse, BuildError> {
        Ok(ExchangeRatesOverridesDeleteReferenceResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}

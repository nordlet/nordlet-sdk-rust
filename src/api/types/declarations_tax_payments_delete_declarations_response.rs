pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaxPaymentsDeleteDeclarationsResponse {
    #[serde(default)]
    pub id: String,
}

impl TaxPaymentsDeleteDeclarationsResponse {
    pub fn builder() -> TaxPaymentsDeleteDeclarationsResponseBuilder {
        <TaxPaymentsDeleteDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaxPaymentsDeleteDeclarationsResponseBuilder {
    id: Option<String>,
}

impl TaxPaymentsDeleteDeclarationsResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaxPaymentsDeleteDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TaxPaymentsDeleteDeclarationsResponseBuilder::id)
    pub fn build(self) -> Result<TaxPaymentsDeleteDeclarationsResponse, BuildError> {
        Ok(TaxPaymentsDeleteDeclarationsResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

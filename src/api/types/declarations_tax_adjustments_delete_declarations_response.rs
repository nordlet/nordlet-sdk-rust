pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaxAdjustmentsDeleteDeclarationsResponse {
    #[serde(default)]
    pub id: String,
}

impl TaxAdjustmentsDeleteDeclarationsResponse {
    pub fn builder() -> TaxAdjustmentsDeleteDeclarationsResponseBuilder {
        <TaxAdjustmentsDeleteDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaxAdjustmentsDeleteDeclarationsResponseBuilder {
    id: Option<String>,
}

impl TaxAdjustmentsDeleteDeclarationsResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaxAdjustmentsDeleteDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TaxAdjustmentsDeleteDeclarationsResponseBuilder::id)
    pub fn build(self) -> Result<TaxAdjustmentsDeleteDeclarationsResponse, BuildError> {
        Ok(TaxAdjustmentsDeleteDeclarationsResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

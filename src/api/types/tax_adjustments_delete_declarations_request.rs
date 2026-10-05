pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaxAdjustmentsDeleteDeclarationsRequest {
    #[serde(default)]
    pub id: String,
}

impl TaxAdjustmentsDeleteDeclarationsRequest {
    pub fn builder() -> TaxAdjustmentsDeleteDeclarationsRequestBuilder {
        <TaxAdjustmentsDeleteDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaxAdjustmentsDeleteDeclarationsRequestBuilder {
    id: Option<String>,
}

impl TaxAdjustmentsDeleteDeclarationsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaxAdjustmentsDeleteDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TaxAdjustmentsDeleteDeclarationsRequestBuilder::id)
    pub fn build(self) -> Result<TaxAdjustmentsDeleteDeclarationsRequest, BuildError> {
        Ok(TaxAdjustmentsDeleteDeclarationsRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

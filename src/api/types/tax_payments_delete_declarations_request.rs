pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaxPaymentsDeleteDeclarationsRequest {
    #[serde(default)]
    pub id: String,
}

impl TaxPaymentsDeleteDeclarationsRequest {
    pub fn builder() -> TaxPaymentsDeleteDeclarationsRequestBuilder {
        <TaxPaymentsDeleteDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaxPaymentsDeleteDeclarationsRequestBuilder {
    id: Option<String>,
}

impl TaxPaymentsDeleteDeclarationsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaxPaymentsDeleteDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TaxPaymentsDeleteDeclarationsRequestBuilder::id)
    pub fn build(self) -> Result<TaxPaymentsDeleteDeclarationsRequest, BuildError> {
        Ok(TaxPaymentsDeleteDeclarationsRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

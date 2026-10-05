pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaxAdjustmentsListDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
}

impl TaxAdjustmentsListDeclarationsRequest {
    pub fn builder() -> TaxAdjustmentsListDeclarationsRequestBuilder {
        <TaxAdjustmentsListDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaxAdjustmentsListDeclarationsRequestBuilder {
    year: Option<i64>,
}

impl TaxAdjustmentsListDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaxAdjustmentsListDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](TaxAdjustmentsListDeclarationsRequestBuilder::year)
    pub fn build(self) -> Result<TaxAdjustmentsListDeclarationsRequest, BuildError> {
        Ok(TaxAdjustmentsListDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}

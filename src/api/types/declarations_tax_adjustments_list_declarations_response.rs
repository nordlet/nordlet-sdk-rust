pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaxAdjustmentsListDeclarationsResponse {
    #[serde(default)]
    pub rows: Vec<TaxAdjustmentsListDeclarationsResponseRowsItem>,
}

impl TaxAdjustmentsListDeclarationsResponse {
    pub fn builder() -> TaxAdjustmentsListDeclarationsResponseBuilder {
        <TaxAdjustmentsListDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaxAdjustmentsListDeclarationsResponseBuilder {
    rows: Option<Vec<TaxAdjustmentsListDeclarationsResponseRowsItem>>,
}

impl TaxAdjustmentsListDeclarationsResponseBuilder {
    pub fn rows(mut self, value: Vec<TaxAdjustmentsListDeclarationsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaxAdjustmentsListDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](TaxAdjustmentsListDeclarationsResponseBuilder::rows)
    pub fn build(self) -> Result<TaxAdjustmentsListDeclarationsResponse, BuildError> {
        Ok(TaxAdjustmentsListDeclarationsResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}

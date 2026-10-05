pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaxPaymentsListDeclarationsResponse {
    #[serde(default)]
    pub rows: Vec<TaxPaymentsListDeclarationsResponseRowsItem>,
}

impl TaxPaymentsListDeclarationsResponse {
    pub fn builder() -> TaxPaymentsListDeclarationsResponseBuilder {
        <TaxPaymentsListDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaxPaymentsListDeclarationsResponseBuilder {
    rows: Option<Vec<TaxPaymentsListDeclarationsResponseRowsItem>>,
}

impl TaxPaymentsListDeclarationsResponseBuilder {
    pub fn rows(mut self, value: Vec<TaxPaymentsListDeclarationsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaxPaymentsListDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](TaxPaymentsListDeclarationsResponseBuilder::rows)
    pub fn build(self) -> Result<TaxPaymentsListDeclarationsResponse, BuildError> {
        Ok(TaxPaymentsListDeclarationsResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}

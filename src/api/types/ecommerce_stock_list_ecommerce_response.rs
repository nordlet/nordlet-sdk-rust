pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StockListEcommerceResponse {
    #[serde(default)]
    pub rows: Vec<StockListEcommerceResponseRowsItem>,
}

impl StockListEcommerceResponse {
    pub fn builder() -> StockListEcommerceResponseBuilder {
        <StockListEcommerceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StockListEcommerceResponseBuilder {
    rows: Option<Vec<StockListEcommerceResponseRowsItem>>,
}

impl StockListEcommerceResponseBuilder {
    pub fn rows(mut self, value: Vec<StockListEcommerceResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StockListEcommerceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](StockListEcommerceResponseBuilder::rows)
    pub fn build(self) -> Result<StockListEcommerceResponse, BuildError> {
        Ok(StockListEcommerceResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}

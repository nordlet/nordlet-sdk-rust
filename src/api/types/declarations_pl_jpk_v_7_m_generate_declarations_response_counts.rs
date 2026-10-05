pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlJpkV7MGenerateDeclarationsResponseCounts {
    #[serde(rename = "salesRows")]
    #[serde(default)]
    pub sales_rows: i64,
    #[serde(rename = "purchaseRows")]
    #[serde(default)]
    pub purchase_rows: i64,
}

impl PlJpkV7MGenerateDeclarationsResponseCounts {
    pub fn builder() -> PlJpkV7MGenerateDeclarationsResponseCountsBuilder {
        <PlJpkV7MGenerateDeclarationsResponseCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlJpkV7MGenerateDeclarationsResponseCountsBuilder {
    sales_rows: Option<i64>,
    purchase_rows: Option<i64>,
}

impl PlJpkV7MGenerateDeclarationsResponseCountsBuilder {
    pub fn sales_rows(mut self, value: i64) -> Self {
        self.sales_rows = Some(value);
        self
    }

    pub fn purchase_rows(mut self, value: i64) -> Self {
        self.purchase_rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlJpkV7MGenerateDeclarationsResponseCounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sales_rows`](PlJpkV7MGenerateDeclarationsResponseCountsBuilder::sales_rows)
    /// - [`purchase_rows`](PlJpkV7MGenerateDeclarationsResponseCountsBuilder::purchase_rows)
    pub fn build(self) -> Result<PlJpkV7MGenerateDeclarationsResponseCounts, BuildError> {
        Ok(PlJpkV7MGenerateDeclarationsResponseCounts {
            sales_rows: self
                .sales_rows
                .ok_or_else(|| BuildError::missing_field("sales_rows"))?,
            purchase_rows: self
                .purchase_rows
                .ok_or_else(|| BuildError::missing_field("purchase_rows"))?,
        })
    }
}

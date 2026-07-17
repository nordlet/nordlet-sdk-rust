pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsStockShortageResponse {
    #[serde(default)]
    pub rows: Vec<PostV1ReportsStockShortageResponseRowsItem>,
}

impl PostV1ReportsStockShortageResponse {
    pub fn builder() -> PostV1ReportsStockShortageResponseBuilder {
        <PostV1ReportsStockShortageResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsStockShortageResponseBuilder {
    rows: Option<Vec<PostV1ReportsStockShortageResponseRowsItem>>,
}

impl PostV1ReportsStockShortageResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1ReportsStockShortageResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsStockShortageResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1ReportsStockShortageResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1ReportsStockShortageResponse, BuildError> {
        Ok(PostV1ReportsStockShortageResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlJpkV7MGenerateResponseCounts {
    #[serde(rename = "salesRows")]
    #[serde(default)]
    pub sales_rows: i64,
    #[serde(rename = "purchaseRows")]
    #[serde(default)]
    pub purchase_rows: i64,
}

impl PostV1DeclarationsPlJpkV7MGenerateResponseCounts {
    pub fn builder() -> PostV1DeclarationsPlJpkV7MGenerateResponseCountsBuilder {
        <PostV1DeclarationsPlJpkV7MGenerateResponseCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlJpkV7MGenerateResponseCountsBuilder {
    sales_rows: Option<i64>,
    purchase_rows: Option<i64>,
}

impl PostV1DeclarationsPlJpkV7MGenerateResponseCountsBuilder {
    pub fn sales_rows(mut self, value: i64) -> Self {
        self.sales_rows = Some(value);
        self
    }

    pub fn purchase_rows(mut self, value: i64) -> Self {
        self.purchase_rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlJpkV7MGenerateResponseCounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sales_rows`](PostV1DeclarationsPlJpkV7MGenerateResponseCountsBuilder::sales_rows)
    /// - [`purchase_rows`](PostV1DeclarationsPlJpkV7MGenerateResponseCountsBuilder::purchase_rows)
    pub fn build(self) -> Result<PostV1DeclarationsPlJpkV7MGenerateResponseCounts, BuildError> {
        Ok(PostV1DeclarationsPlJpkV7MGenerateResponseCounts {
            sales_rows: self
                .sales_rows
                .ok_or_else(|| BuildError::missing_field("sales_rows"))?,
            purchase_rows: self
                .purchase_rows
                .ok_or_else(|| BuildError::missing_field("purchase_rows"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GlDetailReportsResponse {
    #[serde(default)]
    pub account: GlDetailReportsResponseAccount,
    #[serde(default)]
    pub opening: String,
    #[serde(default)]
    pub closing: String,
    #[serde(default)]
    pub rows: Vec<GlDetailReportsResponseRowsItem>,
}

impl GlDetailReportsResponse {
    pub fn builder() -> GlDetailReportsResponseBuilder {
        <GlDetailReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GlDetailReportsResponseBuilder {
    account: Option<GlDetailReportsResponseAccount>,
    opening: Option<String>,
    closing: Option<String>,
    rows: Option<Vec<GlDetailReportsResponseRowsItem>>,
}

impl GlDetailReportsResponseBuilder {
    pub fn account(mut self, value: GlDetailReportsResponseAccount) -> Self {
        self.account = Some(value);
        self
    }

    pub fn opening(mut self, value: impl Into<String>) -> Self {
        self.opening = Some(value.into());
        self
    }

    pub fn closing(mut self, value: impl Into<String>) -> Self {
        self.closing = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<GlDetailReportsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GlDetailReportsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account`](GlDetailReportsResponseBuilder::account)
    /// - [`opening`](GlDetailReportsResponseBuilder::opening)
    /// - [`closing`](GlDetailReportsResponseBuilder::closing)
    /// - [`rows`](GlDetailReportsResponseBuilder::rows)
    pub fn build(self) -> Result<GlDetailReportsResponse, BuildError> {
        Ok(GlDetailReportsResponse {
            account: self
                .account
                .ok_or_else(|| BuildError::missing_field("account"))?,
            opening: self
                .opening
                .ok_or_else(|| BuildError::missing_field("opening"))?,
            closing: self
                .closing
                .ok_or_else(|| BuildError::missing_field("closing"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}

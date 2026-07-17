pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsGlDetailResponse {
    #[serde(default)]
    pub account: PostV1ReportsGlDetailResponseAccount,
    #[serde(default)]
    pub opening: String,
    #[serde(default)]
    pub closing: String,
    #[serde(default)]
    pub rows: Vec<PostV1ReportsGlDetailResponseRowsItem>,
}

impl PostV1ReportsGlDetailResponse {
    pub fn builder() -> PostV1ReportsGlDetailResponseBuilder {
        <PostV1ReportsGlDetailResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsGlDetailResponseBuilder {
    account: Option<PostV1ReportsGlDetailResponseAccount>,
    opening: Option<String>,
    closing: Option<String>,
    rows: Option<Vec<PostV1ReportsGlDetailResponseRowsItem>>,
}

impl PostV1ReportsGlDetailResponseBuilder {
    pub fn account(mut self, value: PostV1ReportsGlDetailResponseAccount) -> Self {
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

    pub fn rows(mut self, value: Vec<PostV1ReportsGlDetailResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsGlDetailResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account`](PostV1ReportsGlDetailResponseBuilder::account)
    /// - [`opening`](PostV1ReportsGlDetailResponseBuilder::opening)
    /// - [`closing`](PostV1ReportsGlDetailResponseBuilder::closing)
    /// - [`rows`](PostV1ReportsGlDetailResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1ReportsGlDetailResponse, BuildError> {
        Ok(PostV1ReportsGlDetailResponse {
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

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtIntrastatObligationResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "isVatPayer")]
    #[serde(default)]
    pub is_vat_payer: bool,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub thresholds: PostV1DeclarationsLtIntrastatObligationResponseThresholds,
    #[serde(default)]
    pub arrivals: PostV1DeclarationsLtIntrastatObligationResponseArrivals,
    #[serde(default)]
    pub dispatches: PostV1DeclarationsLtIntrastatObligationResponseDispatches,
}

impl PostV1DeclarationsLtIntrastatObligationResponse {
    pub fn builder() -> PostV1DeclarationsLtIntrastatObligationResponseBuilder {
        <PostV1DeclarationsLtIntrastatObligationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtIntrastatObligationResponseBuilder {
    year: Option<i64>,
    is_vat_payer: Option<bool>,
    notes: Option<Vec<String>>,
    thresholds: Option<PostV1DeclarationsLtIntrastatObligationResponseThresholds>,
    arrivals: Option<PostV1DeclarationsLtIntrastatObligationResponseArrivals>,
    dispatches: Option<PostV1DeclarationsLtIntrastatObligationResponseDispatches>,
}

impl PostV1DeclarationsLtIntrastatObligationResponseBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn is_vat_payer(mut self, value: bool) -> Self {
        self.is_vat_payer = Some(value);
        self
    }

    pub fn notes(mut self, value: Vec<String>) -> Self {
        self.notes = Some(value);
        self
    }

    pub fn thresholds(
        mut self,
        value: PostV1DeclarationsLtIntrastatObligationResponseThresholds,
    ) -> Self {
        self.thresholds = Some(value);
        self
    }

    pub fn arrivals(
        mut self,
        value: PostV1DeclarationsLtIntrastatObligationResponseArrivals,
    ) -> Self {
        self.arrivals = Some(value);
        self
    }

    pub fn dispatches(
        mut self,
        value: PostV1DeclarationsLtIntrastatObligationResponseDispatches,
    ) -> Self {
        self.dispatches = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtIntrastatObligationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsLtIntrastatObligationResponseBuilder::year)
    /// - [`is_vat_payer`](PostV1DeclarationsLtIntrastatObligationResponseBuilder::is_vat_payer)
    /// - [`notes`](PostV1DeclarationsLtIntrastatObligationResponseBuilder::notes)
    /// - [`thresholds`](PostV1DeclarationsLtIntrastatObligationResponseBuilder::thresholds)
    /// - [`arrivals`](PostV1DeclarationsLtIntrastatObligationResponseBuilder::arrivals)
    /// - [`dispatches`](PostV1DeclarationsLtIntrastatObligationResponseBuilder::dispatches)
    pub fn build(self) -> Result<PostV1DeclarationsLtIntrastatObligationResponse, BuildError> {
        Ok(PostV1DeclarationsLtIntrastatObligationResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            is_vat_payer: self
                .is_vat_payer
                .ok_or_else(|| BuildError::missing_field("is_vat_payer"))?,
            notes: self
                .notes
                .ok_or_else(|| BuildError::missing_field("notes"))?,
            thresholds: self
                .thresholds
                .ok_or_else(|| BuildError::missing_field("thresholds"))?,
            arrivals: self
                .arrivals
                .ok_or_else(|| BuildError::missing_field("arrivals"))?,
            dispatches: self
                .dispatches
                .ok_or_else(|| BuildError::missing_field("dispatches"))?,
        })
    }
}

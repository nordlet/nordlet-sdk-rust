pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtIntrastatObligationDeclarationsResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "isVatPayer")]
    #[serde(default)]
    pub is_vat_payer: bool,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub thresholds: LtIntrastatObligationDeclarationsResponseThresholds,
    #[serde(default)]
    pub arrivals: LtIntrastatObligationDeclarationsResponseArrivals,
    #[serde(default)]
    pub dispatches: LtIntrastatObligationDeclarationsResponseDispatches,
}

impl LtIntrastatObligationDeclarationsResponse {
    pub fn builder() -> LtIntrastatObligationDeclarationsResponseBuilder {
        <LtIntrastatObligationDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtIntrastatObligationDeclarationsResponseBuilder {
    year: Option<i64>,
    is_vat_payer: Option<bool>,
    notes: Option<Vec<String>>,
    thresholds: Option<LtIntrastatObligationDeclarationsResponseThresholds>,
    arrivals: Option<LtIntrastatObligationDeclarationsResponseArrivals>,
    dispatches: Option<LtIntrastatObligationDeclarationsResponseDispatches>,
}

impl LtIntrastatObligationDeclarationsResponseBuilder {
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
        value: LtIntrastatObligationDeclarationsResponseThresholds,
    ) -> Self {
        self.thresholds = Some(value);
        self
    }

    pub fn arrivals(mut self, value: LtIntrastatObligationDeclarationsResponseArrivals) -> Self {
        self.arrivals = Some(value);
        self
    }

    pub fn dispatches(
        mut self,
        value: LtIntrastatObligationDeclarationsResponseDispatches,
    ) -> Self {
        self.dispatches = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtIntrastatObligationDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](LtIntrastatObligationDeclarationsResponseBuilder::year)
    /// - [`is_vat_payer`](LtIntrastatObligationDeclarationsResponseBuilder::is_vat_payer)
    /// - [`notes`](LtIntrastatObligationDeclarationsResponseBuilder::notes)
    /// - [`thresholds`](LtIntrastatObligationDeclarationsResponseBuilder::thresholds)
    /// - [`arrivals`](LtIntrastatObligationDeclarationsResponseBuilder::arrivals)
    /// - [`dispatches`](LtIntrastatObligationDeclarationsResponseBuilder::dispatches)
    pub fn build(self) -> Result<LtIntrastatObligationDeclarationsResponse, BuildError> {
        Ok(LtIntrastatObligationDeclarationsResponse {
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

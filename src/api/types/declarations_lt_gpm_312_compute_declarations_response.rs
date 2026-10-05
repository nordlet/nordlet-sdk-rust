pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LtGpm312ComputeDeclarationsResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "payoutTiming")]
    pub payout_timing: LtGpm312ComputeDeclarationsResponsePayoutTiming,
    #[serde(rename = "payoutFrom")]
    #[serde(default)]
    pub payout_from: LtGpm312ComputeDeclarationsResponsePayoutFrom,
    #[serde(rename = "payoutTo")]
    #[serde(default)]
    pub payout_to: LtGpm312ComputeDeclarationsResponsePayoutTo,
    #[serde(rename = "registrationNumber")]
    #[serde(default)]
    pub registration_number: String,
    #[serde(rename = "companyName")]
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub rows: Vec<LtGpm312ComputeDeclarationsResponseRowsItem>,
    #[serde(default)]
    pub totals: LtGpm312ComputeDeclarationsResponseTotals,
    #[serde(rename = "runsFound")]
    #[serde(default)]
    pub runs_found: i64,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl LtGpm312ComputeDeclarationsResponse {
    pub fn builder() -> LtGpm312ComputeDeclarationsResponseBuilder {
        <LtGpm312ComputeDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtGpm312ComputeDeclarationsResponseBuilder {
    year: Option<i64>,
    payout_timing: Option<LtGpm312ComputeDeclarationsResponsePayoutTiming>,
    payout_from: Option<LtGpm312ComputeDeclarationsResponsePayoutFrom>,
    payout_to: Option<LtGpm312ComputeDeclarationsResponsePayoutTo>,
    registration_number: Option<String>,
    company_name: Option<String>,
    rows: Option<Vec<LtGpm312ComputeDeclarationsResponseRowsItem>>,
    totals: Option<LtGpm312ComputeDeclarationsResponseTotals>,
    runs_found: Option<i64>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
}

impl LtGpm312ComputeDeclarationsResponseBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn payout_timing(mut self, value: LtGpm312ComputeDeclarationsResponsePayoutTiming) -> Self {
        self.payout_timing = Some(value);
        self
    }

    pub fn payout_from(mut self, value: LtGpm312ComputeDeclarationsResponsePayoutFrom) -> Self {
        self.payout_from = Some(value);
        self
    }

    pub fn payout_to(mut self, value: LtGpm312ComputeDeclarationsResponsePayoutTo) -> Self {
        self.payout_to = Some(value);
        self
    }

    pub fn registration_number(mut self, value: impl Into<String>) -> Self {
        self.registration_number = Some(value.into());
        self
    }

    pub fn company_name(mut self, value: impl Into<String>) -> Self {
        self.company_name = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<LtGpm312ComputeDeclarationsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn totals(mut self, value: LtGpm312ComputeDeclarationsResponseTotals) -> Self {
        self.totals = Some(value);
        self
    }

    pub fn runs_found(mut self, value: i64) -> Self {
        self.runs_found = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    pub fn notes(mut self, value: Vec<String>) -> Self {
        self.notes = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LtGpm312ComputeDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](LtGpm312ComputeDeclarationsResponseBuilder::year)
    /// - [`payout_timing`](LtGpm312ComputeDeclarationsResponseBuilder::payout_timing)
    /// - [`payout_from`](LtGpm312ComputeDeclarationsResponseBuilder::payout_from)
    /// - [`payout_to`](LtGpm312ComputeDeclarationsResponseBuilder::payout_to)
    /// - [`registration_number`](LtGpm312ComputeDeclarationsResponseBuilder::registration_number)
    /// - [`company_name`](LtGpm312ComputeDeclarationsResponseBuilder::company_name)
    /// - [`rows`](LtGpm312ComputeDeclarationsResponseBuilder::rows)
    /// - [`totals`](LtGpm312ComputeDeclarationsResponseBuilder::totals)
    /// - [`runs_found`](LtGpm312ComputeDeclarationsResponseBuilder::runs_found)
    /// - [`warnings`](LtGpm312ComputeDeclarationsResponseBuilder::warnings)
    /// - [`notes`](LtGpm312ComputeDeclarationsResponseBuilder::notes)
    /// - [`source`](LtGpm312ComputeDeclarationsResponseBuilder::source)
    pub fn build(self) -> Result<LtGpm312ComputeDeclarationsResponse, BuildError> {
        Ok(LtGpm312ComputeDeclarationsResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            payout_timing: self
                .payout_timing
                .ok_or_else(|| BuildError::missing_field("payout_timing"))?,
            payout_from: self
                .payout_from
                .ok_or_else(|| BuildError::missing_field("payout_from"))?,
            payout_to: self
                .payout_to
                .ok_or_else(|| BuildError::missing_field("payout_to"))?,
            registration_number: self
                .registration_number
                .ok_or_else(|| BuildError::missing_field("registration_number"))?,
            company_name: self
                .company_name
                .ok_or_else(|| BuildError::missing_field("company_name"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            totals: self
                .totals
                .ok_or_else(|| BuildError::missing_field("totals"))?,
            runs_found: self
                .runs_found
                .ok_or_else(|| BuildError::missing_field("runs_found"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            notes: self
                .notes
                .ok_or_else(|| BuildError::missing_field("notes"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}

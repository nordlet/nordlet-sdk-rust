pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtGpm312ComputeResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "payoutTiming")]
    pub payout_timing: PostV1DeclarationsLtGpm312ComputeResponsePayoutTiming,
    #[serde(rename = "payoutFrom")]
    #[serde(default)]
    pub payout_from: PostV1DeclarationsLtGpm312ComputeResponsePayoutFrom,
    #[serde(rename = "payoutTo")]
    #[serde(default)]
    pub payout_to: PostV1DeclarationsLtGpm312ComputeResponsePayoutTo,
    #[serde(rename = "registrationNumber")]
    #[serde(default)]
    pub registration_number: String,
    #[serde(rename = "companyName")]
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub rows: Vec<PostV1DeclarationsLtGpm312ComputeResponseRowsItem>,
    #[serde(default)]
    pub totals: PostV1DeclarationsLtGpm312ComputeResponseTotals,
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

impl PostV1DeclarationsLtGpm312ComputeResponse {
    pub fn builder() -> PostV1DeclarationsLtGpm312ComputeResponseBuilder {
        <PostV1DeclarationsLtGpm312ComputeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtGpm312ComputeResponseBuilder {
    year: Option<i64>,
    payout_timing: Option<PostV1DeclarationsLtGpm312ComputeResponsePayoutTiming>,
    payout_from: Option<PostV1DeclarationsLtGpm312ComputeResponsePayoutFrom>,
    payout_to: Option<PostV1DeclarationsLtGpm312ComputeResponsePayoutTo>,
    registration_number: Option<String>,
    company_name: Option<String>,
    rows: Option<Vec<PostV1DeclarationsLtGpm312ComputeResponseRowsItem>>,
    totals: Option<PostV1DeclarationsLtGpm312ComputeResponseTotals>,
    runs_found: Option<i64>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
}

impl PostV1DeclarationsLtGpm312ComputeResponseBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn payout_timing(
        mut self,
        value: PostV1DeclarationsLtGpm312ComputeResponsePayoutTiming,
    ) -> Self {
        self.payout_timing = Some(value);
        self
    }

    pub fn payout_from(
        mut self,
        value: PostV1DeclarationsLtGpm312ComputeResponsePayoutFrom,
    ) -> Self {
        self.payout_from = Some(value);
        self
    }

    pub fn payout_to(mut self, value: PostV1DeclarationsLtGpm312ComputeResponsePayoutTo) -> Self {
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

    pub fn rows(mut self, value: Vec<PostV1DeclarationsLtGpm312ComputeResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn totals(mut self, value: PostV1DeclarationsLtGpm312ComputeResponseTotals) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtGpm312ComputeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsLtGpm312ComputeResponseBuilder::year)
    /// - [`payout_timing`](PostV1DeclarationsLtGpm312ComputeResponseBuilder::payout_timing)
    /// - [`payout_from`](PostV1DeclarationsLtGpm312ComputeResponseBuilder::payout_from)
    /// - [`payout_to`](PostV1DeclarationsLtGpm312ComputeResponseBuilder::payout_to)
    /// - [`registration_number`](PostV1DeclarationsLtGpm312ComputeResponseBuilder::registration_number)
    /// - [`company_name`](PostV1DeclarationsLtGpm312ComputeResponseBuilder::company_name)
    /// - [`rows`](PostV1DeclarationsLtGpm312ComputeResponseBuilder::rows)
    /// - [`totals`](PostV1DeclarationsLtGpm312ComputeResponseBuilder::totals)
    /// - [`runs_found`](PostV1DeclarationsLtGpm312ComputeResponseBuilder::runs_found)
    /// - [`warnings`](PostV1DeclarationsLtGpm312ComputeResponseBuilder::warnings)
    /// - [`notes`](PostV1DeclarationsLtGpm312ComputeResponseBuilder::notes)
    /// - [`source`](PostV1DeclarationsLtGpm312ComputeResponseBuilder::source)
    pub fn build(self) -> Result<PostV1DeclarationsLtGpm312ComputeResponse, BuildError> {
        Ok(PostV1DeclarationsLtGpm312ComputeResponse {
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

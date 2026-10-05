pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlVatUeGenerateDeclarationsResponse {
    #[serde(rename = "periodStart")]
    #[serde(default)]
    pub period_start: String,
    #[serde(rename = "periodEnd")]
    #[serde(default)]
    pub period_end: String,
    #[serde(default)]
    pub nip: String,
    #[serde(rename = "companyName")]
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub rows: Vec<PlVatUeGenerateDeclarationsResponseRowsItem>,
    #[serde(default)]
    pub totals: Vec<PlVatUeGenerateDeclarationsResponseTotalsItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl PlVatUeGenerateDeclarationsResponse {
    pub fn builder() -> PlVatUeGenerateDeclarationsResponseBuilder {
        <PlVatUeGenerateDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlVatUeGenerateDeclarationsResponseBuilder {
    period_start: Option<String>,
    period_end: Option<String>,
    nip: Option<String>,
    company_name: Option<String>,
    rows: Option<Vec<PlVatUeGenerateDeclarationsResponseRowsItem>>,
    totals: Option<Vec<PlVatUeGenerateDeclarationsResponseTotalsItem>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
}

impl PlVatUeGenerateDeclarationsResponseBuilder {
    pub fn period_start(mut self, value: impl Into<String>) -> Self {
        self.period_start = Some(value.into());
        self
    }

    pub fn period_end(mut self, value: impl Into<String>) -> Self {
        self.period_end = Some(value.into());
        self
    }

    pub fn nip(mut self, value: impl Into<String>) -> Self {
        self.nip = Some(value.into());
        self
    }

    pub fn company_name(mut self, value: impl Into<String>) -> Self {
        self.company_name = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<PlVatUeGenerateDeclarationsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn totals(mut self, value: Vec<PlVatUeGenerateDeclarationsResponseTotalsItem>) -> Self {
        self.totals = Some(value);
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

    /// Consumes the builder and constructs a [`PlVatUeGenerateDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`period_start`](PlVatUeGenerateDeclarationsResponseBuilder::period_start)
    /// - [`period_end`](PlVatUeGenerateDeclarationsResponseBuilder::period_end)
    /// - [`nip`](PlVatUeGenerateDeclarationsResponseBuilder::nip)
    /// - [`company_name`](PlVatUeGenerateDeclarationsResponseBuilder::company_name)
    /// - [`rows`](PlVatUeGenerateDeclarationsResponseBuilder::rows)
    /// - [`totals`](PlVatUeGenerateDeclarationsResponseBuilder::totals)
    /// - [`warnings`](PlVatUeGenerateDeclarationsResponseBuilder::warnings)
    /// - [`notes`](PlVatUeGenerateDeclarationsResponseBuilder::notes)
    /// - [`source`](PlVatUeGenerateDeclarationsResponseBuilder::source)
    pub fn build(self) -> Result<PlVatUeGenerateDeclarationsResponse, BuildError> {
        Ok(PlVatUeGenerateDeclarationsResponse {
            period_start: self
                .period_start
                .ok_or_else(|| BuildError::missing_field("period_start"))?,
            period_end: self
                .period_end
                .ok_or_else(|| BuildError::missing_field("period_end"))?,
            nip: self.nip.ok_or_else(|| BuildError::missing_field("nip"))?,
            company_name: self
                .company_name
                .ok_or_else(|| BuildError::missing_field("company_name"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            totals: self
                .totals
                .ok_or_else(|| BuildError::missing_field("totals"))?,
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

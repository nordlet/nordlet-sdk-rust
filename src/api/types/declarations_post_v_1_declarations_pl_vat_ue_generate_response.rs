pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlVatUeGenerateResponse {
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
    pub rows: Vec<PostV1DeclarationsPlVatUeGenerateResponseRowsItem>,
    #[serde(default)]
    pub totals: Vec<PostV1DeclarationsPlVatUeGenerateResponseTotalsItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl PostV1DeclarationsPlVatUeGenerateResponse {
    pub fn builder() -> PostV1DeclarationsPlVatUeGenerateResponseBuilder {
        <PostV1DeclarationsPlVatUeGenerateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlVatUeGenerateResponseBuilder {
    period_start: Option<String>,
    period_end: Option<String>,
    nip: Option<String>,
    company_name: Option<String>,
    rows: Option<Vec<PostV1DeclarationsPlVatUeGenerateResponseRowsItem>>,
    totals: Option<Vec<PostV1DeclarationsPlVatUeGenerateResponseTotalsItem>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
}

impl PostV1DeclarationsPlVatUeGenerateResponseBuilder {
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

    pub fn rows(mut self, value: Vec<PostV1DeclarationsPlVatUeGenerateResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn totals(
        mut self,
        value: Vec<PostV1DeclarationsPlVatUeGenerateResponseTotalsItem>,
    ) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlVatUeGenerateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`period_start`](PostV1DeclarationsPlVatUeGenerateResponseBuilder::period_start)
    /// - [`period_end`](PostV1DeclarationsPlVatUeGenerateResponseBuilder::period_end)
    /// - [`nip`](PostV1DeclarationsPlVatUeGenerateResponseBuilder::nip)
    /// - [`company_name`](PostV1DeclarationsPlVatUeGenerateResponseBuilder::company_name)
    /// - [`rows`](PostV1DeclarationsPlVatUeGenerateResponseBuilder::rows)
    /// - [`totals`](PostV1DeclarationsPlVatUeGenerateResponseBuilder::totals)
    /// - [`warnings`](PostV1DeclarationsPlVatUeGenerateResponseBuilder::warnings)
    /// - [`notes`](PostV1DeclarationsPlVatUeGenerateResponseBuilder::notes)
    /// - [`source`](PostV1DeclarationsPlVatUeGenerateResponseBuilder::source)
    pub fn build(self) -> Result<PostV1DeclarationsPlVatUeGenerateResponse, BuildError> {
        Ok(PostV1DeclarationsPlVatUeGenerateResponse {
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

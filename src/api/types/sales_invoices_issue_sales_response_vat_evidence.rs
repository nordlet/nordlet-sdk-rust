pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesIssueSalesResponseVatEvidence {
    #[serde(rename = "capturedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub captured_at: DateTime<FixedOffset>,
    #[serde(rename = "issueDate")]
    #[serde(default)]
    pub issue_date: NaiveDate,
    #[serde(default)]
    pub scheme: InvoicesIssueSalesResponseVatEvidenceScheme,
    #[serde(default)]
    pub partner: InvoicesIssueSalesResponseVatEvidencePartner,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vies: Option<InvoicesIssueSalesResponseVatEvidenceVies>,
    #[serde(default)]
    pub location: InvoicesIssueSalesResponseVatEvidenceLocation,
    #[serde(rename = "rateTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_table: Option<InvoicesIssueSalesResponseVatEvidenceRateTable>,
    #[serde(default)]
    pub rates: Vec<InvoicesIssueSalesResponseVatEvidenceRatesItem>,
}

impl InvoicesIssueSalesResponseVatEvidence {
    pub fn builder() -> InvoicesIssueSalesResponseVatEvidenceBuilder {
        <InvoicesIssueSalesResponseVatEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesIssueSalesResponseVatEvidenceBuilder {
    captured_at: Option<DateTime<FixedOffset>>,
    issue_date: Option<NaiveDate>,
    scheme: Option<InvoicesIssueSalesResponseVatEvidenceScheme>,
    partner: Option<InvoicesIssueSalesResponseVatEvidencePartner>,
    vies: Option<InvoicesIssueSalesResponseVatEvidenceVies>,
    location: Option<InvoicesIssueSalesResponseVatEvidenceLocation>,
    rate_table: Option<InvoicesIssueSalesResponseVatEvidenceRateTable>,
    rates: Option<Vec<InvoicesIssueSalesResponseVatEvidenceRatesItem>>,
}

impl InvoicesIssueSalesResponseVatEvidenceBuilder {
    pub fn captured_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.captured_at = Some(value);
        self
    }

    pub fn issue_date(mut self, value: NaiveDate) -> Self {
        self.issue_date = Some(value);
        self
    }

    pub fn scheme(mut self, value: InvoicesIssueSalesResponseVatEvidenceScheme) -> Self {
        self.scheme = Some(value);
        self
    }

    pub fn partner(mut self, value: InvoicesIssueSalesResponseVatEvidencePartner) -> Self {
        self.partner = Some(value);
        self
    }

    pub fn vies(mut self, value: InvoicesIssueSalesResponseVatEvidenceVies) -> Self {
        self.vies = Some(value);
        self
    }

    pub fn location(mut self, value: InvoicesIssueSalesResponseVatEvidenceLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn rate_table(mut self, value: InvoicesIssueSalesResponseVatEvidenceRateTable) -> Self {
        self.rate_table = Some(value);
        self
    }

    pub fn rates(mut self, value: Vec<InvoicesIssueSalesResponseVatEvidenceRatesItem>) -> Self {
        self.rates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoicesIssueSalesResponseVatEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`captured_at`](InvoicesIssueSalesResponseVatEvidenceBuilder::captured_at)
    /// - [`issue_date`](InvoicesIssueSalesResponseVatEvidenceBuilder::issue_date)
    /// - [`scheme`](InvoicesIssueSalesResponseVatEvidenceBuilder::scheme)
    /// - [`partner`](InvoicesIssueSalesResponseVatEvidenceBuilder::partner)
    /// - [`location`](InvoicesIssueSalesResponseVatEvidenceBuilder::location)
    /// - [`rates`](InvoicesIssueSalesResponseVatEvidenceBuilder::rates)
    pub fn build(self) -> Result<InvoicesIssueSalesResponseVatEvidence, BuildError> {
        Ok(InvoicesIssueSalesResponseVatEvidence {
            captured_at: self
                .captured_at
                .ok_or_else(|| BuildError::missing_field("captured_at"))?,
            issue_date: self
                .issue_date
                .ok_or_else(|| BuildError::missing_field("issue_date"))?,
            scheme: self
                .scheme
                .ok_or_else(|| BuildError::missing_field("scheme"))?,
            partner: self
                .partner
                .ok_or_else(|| BuildError::missing_field("partner"))?,
            vies: self.vies,
            location: self
                .location
                .ok_or_else(|| BuildError::missing_field("location"))?,
            rate_table: self.rate_table,
            rates: self
                .rates
                .ok_or_else(|| BuildError::missing_field("rates"))?,
        })
    }
}

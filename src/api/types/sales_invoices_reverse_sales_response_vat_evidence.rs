pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesReverseSalesResponseVatEvidence {
    #[serde(rename = "capturedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub captured_at: DateTime<FixedOffset>,
    #[serde(rename = "issueDate")]
    #[serde(default)]
    pub issue_date: NaiveDate,
    #[serde(default)]
    pub scheme: InvoicesReverseSalesResponseVatEvidenceScheme,
    #[serde(default)]
    pub partner: InvoicesReverseSalesResponseVatEvidencePartner,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vies: Option<InvoicesReverseSalesResponseVatEvidenceVies>,
    #[serde(default)]
    pub location: InvoicesReverseSalesResponseVatEvidenceLocation,
    #[serde(rename = "rateTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_table: Option<InvoicesReverseSalesResponseVatEvidenceRateTable>,
    #[serde(default)]
    pub rates: Vec<InvoicesReverseSalesResponseVatEvidenceRatesItem>,
}

impl InvoicesReverseSalesResponseVatEvidence {
    pub fn builder() -> InvoicesReverseSalesResponseVatEvidenceBuilder {
        <InvoicesReverseSalesResponseVatEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesReverseSalesResponseVatEvidenceBuilder {
    captured_at: Option<DateTime<FixedOffset>>,
    issue_date: Option<NaiveDate>,
    scheme: Option<InvoicesReverseSalesResponseVatEvidenceScheme>,
    partner: Option<InvoicesReverseSalesResponseVatEvidencePartner>,
    vies: Option<InvoicesReverseSalesResponseVatEvidenceVies>,
    location: Option<InvoicesReverseSalesResponseVatEvidenceLocation>,
    rate_table: Option<InvoicesReverseSalesResponseVatEvidenceRateTable>,
    rates: Option<Vec<InvoicesReverseSalesResponseVatEvidenceRatesItem>>,
}

impl InvoicesReverseSalesResponseVatEvidenceBuilder {
    pub fn captured_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.captured_at = Some(value);
        self
    }

    pub fn issue_date(mut self, value: NaiveDate) -> Self {
        self.issue_date = Some(value);
        self
    }

    pub fn scheme(mut self, value: InvoicesReverseSalesResponseVatEvidenceScheme) -> Self {
        self.scheme = Some(value);
        self
    }

    pub fn partner(mut self, value: InvoicesReverseSalesResponseVatEvidencePartner) -> Self {
        self.partner = Some(value);
        self
    }

    pub fn vies(mut self, value: InvoicesReverseSalesResponseVatEvidenceVies) -> Self {
        self.vies = Some(value);
        self
    }

    pub fn location(mut self, value: InvoicesReverseSalesResponseVatEvidenceLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn rate_table(mut self, value: InvoicesReverseSalesResponseVatEvidenceRateTable) -> Self {
        self.rate_table = Some(value);
        self
    }

    pub fn rates(mut self, value: Vec<InvoicesReverseSalesResponseVatEvidenceRatesItem>) -> Self {
        self.rates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoicesReverseSalesResponseVatEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`captured_at`](InvoicesReverseSalesResponseVatEvidenceBuilder::captured_at)
    /// - [`issue_date`](InvoicesReverseSalesResponseVatEvidenceBuilder::issue_date)
    /// - [`scheme`](InvoicesReverseSalesResponseVatEvidenceBuilder::scheme)
    /// - [`partner`](InvoicesReverseSalesResponseVatEvidenceBuilder::partner)
    /// - [`location`](InvoicesReverseSalesResponseVatEvidenceBuilder::location)
    /// - [`rates`](InvoicesReverseSalesResponseVatEvidenceBuilder::rates)
    pub fn build(self) -> Result<InvoicesReverseSalesResponseVatEvidence, BuildError> {
        Ok(InvoicesReverseSalesResponseVatEvidence {
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

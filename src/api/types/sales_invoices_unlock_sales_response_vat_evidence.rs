pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesUnlockSalesResponseVatEvidence {
    #[serde(rename = "capturedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub captured_at: DateTime<FixedOffset>,
    #[serde(rename = "issueDate")]
    #[serde(default)]
    pub issue_date: NaiveDate,
    #[serde(default)]
    pub scheme: InvoicesUnlockSalesResponseVatEvidenceScheme,
    #[serde(default)]
    pub partner: InvoicesUnlockSalesResponseVatEvidencePartner,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vies: Option<InvoicesUnlockSalesResponseVatEvidenceVies>,
    #[serde(default)]
    pub location: InvoicesUnlockSalesResponseVatEvidenceLocation,
    #[serde(rename = "rateTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_table: Option<InvoicesUnlockSalesResponseVatEvidenceRateTable>,
    #[serde(default)]
    pub rates: Vec<InvoicesUnlockSalesResponseVatEvidenceRatesItem>,
}

impl InvoicesUnlockSalesResponseVatEvidence {
    pub fn builder() -> InvoicesUnlockSalesResponseVatEvidenceBuilder {
        <InvoicesUnlockSalesResponseVatEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesUnlockSalesResponseVatEvidenceBuilder {
    captured_at: Option<DateTime<FixedOffset>>,
    issue_date: Option<NaiveDate>,
    scheme: Option<InvoicesUnlockSalesResponseVatEvidenceScheme>,
    partner: Option<InvoicesUnlockSalesResponseVatEvidencePartner>,
    vies: Option<InvoicesUnlockSalesResponseVatEvidenceVies>,
    location: Option<InvoicesUnlockSalesResponseVatEvidenceLocation>,
    rate_table: Option<InvoicesUnlockSalesResponseVatEvidenceRateTable>,
    rates: Option<Vec<InvoicesUnlockSalesResponseVatEvidenceRatesItem>>,
}

impl InvoicesUnlockSalesResponseVatEvidenceBuilder {
    pub fn captured_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.captured_at = Some(value);
        self
    }

    pub fn issue_date(mut self, value: NaiveDate) -> Self {
        self.issue_date = Some(value);
        self
    }

    pub fn scheme(mut self, value: InvoicesUnlockSalesResponseVatEvidenceScheme) -> Self {
        self.scheme = Some(value);
        self
    }

    pub fn partner(mut self, value: InvoicesUnlockSalesResponseVatEvidencePartner) -> Self {
        self.partner = Some(value);
        self
    }

    pub fn vies(mut self, value: InvoicesUnlockSalesResponseVatEvidenceVies) -> Self {
        self.vies = Some(value);
        self
    }

    pub fn location(mut self, value: InvoicesUnlockSalesResponseVatEvidenceLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn rate_table(mut self, value: InvoicesUnlockSalesResponseVatEvidenceRateTable) -> Self {
        self.rate_table = Some(value);
        self
    }

    pub fn rates(mut self, value: Vec<InvoicesUnlockSalesResponseVatEvidenceRatesItem>) -> Self {
        self.rates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoicesUnlockSalesResponseVatEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`captured_at`](InvoicesUnlockSalesResponseVatEvidenceBuilder::captured_at)
    /// - [`issue_date`](InvoicesUnlockSalesResponseVatEvidenceBuilder::issue_date)
    /// - [`scheme`](InvoicesUnlockSalesResponseVatEvidenceBuilder::scheme)
    /// - [`partner`](InvoicesUnlockSalesResponseVatEvidenceBuilder::partner)
    /// - [`location`](InvoicesUnlockSalesResponseVatEvidenceBuilder::location)
    /// - [`rates`](InvoicesUnlockSalesResponseVatEvidenceBuilder::rates)
    pub fn build(self) -> Result<InvoicesUnlockSalesResponseVatEvidence, BuildError> {
        Ok(InvoicesUnlockSalesResponseVatEvidence {
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

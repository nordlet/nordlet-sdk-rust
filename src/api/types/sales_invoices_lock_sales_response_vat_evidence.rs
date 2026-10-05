pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesLockSalesResponseVatEvidence {
    #[serde(rename = "capturedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub captured_at: DateTime<FixedOffset>,
    #[serde(rename = "issueDate")]
    #[serde(default)]
    pub issue_date: NaiveDate,
    #[serde(default)]
    pub scheme: InvoicesLockSalesResponseVatEvidenceScheme,
    #[serde(default)]
    pub partner: InvoicesLockSalesResponseVatEvidencePartner,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vies: Option<InvoicesLockSalesResponseVatEvidenceVies>,
    #[serde(default)]
    pub location: InvoicesLockSalesResponseVatEvidenceLocation,
    #[serde(rename = "rateTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_table: Option<InvoicesLockSalesResponseVatEvidenceRateTable>,
    #[serde(default)]
    pub rates: Vec<InvoicesLockSalesResponseVatEvidenceRatesItem>,
}

impl InvoicesLockSalesResponseVatEvidence {
    pub fn builder() -> InvoicesLockSalesResponseVatEvidenceBuilder {
        <InvoicesLockSalesResponseVatEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesLockSalesResponseVatEvidenceBuilder {
    captured_at: Option<DateTime<FixedOffset>>,
    issue_date: Option<NaiveDate>,
    scheme: Option<InvoicesLockSalesResponseVatEvidenceScheme>,
    partner: Option<InvoicesLockSalesResponseVatEvidencePartner>,
    vies: Option<InvoicesLockSalesResponseVatEvidenceVies>,
    location: Option<InvoicesLockSalesResponseVatEvidenceLocation>,
    rate_table: Option<InvoicesLockSalesResponseVatEvidenceRateTable>,
    rates: Option<Vec<InvoicesLockSalesResponseVatEvidenceRatesItem>>,
}

impl InvoicesLockSalesResponseVatEvidenceBuilder {
    pub fn captured_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.captured_at = Some(value);
        self
    }

    pub fn issue_date(mut self, value: NaiveDate) -> Self {
        self.issue_date = Some(value);
        self
    }

    pub fn scheme(mut self, value: InvoicesLockSalesResponseVatEvidenceScheme) -> Self {
        self.scheme = Some(value);
        self
    }

    pub fn partner(mut self, value: InvoicesLockSalesResponseVatEvidencePartner) -> Self {
        self.partner = Some(value);
        self
    }

    pub fn vies(mut self, value: InvoicesLockSalesResponseVatEvidenceVies) -> Self {
        self.vies = Some(value);
        self
    }

    pub fn location(mut self, value: InvoicesLockSalesResponseVatEvidenceLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn rate_table(mut self, value: InvoicesLockSalesResponseVatEvidenceRateTable) -> Self {
        self.rate_table = Some(value);
        self
    }

    pub fn rates(mut self, value: Vec<InvoicesLockSalesResponseVatEvidenceRatesItem>) -> Self {
        self.rates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoicesLockSalesResponseVatEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`captured_at`](InvoicesLockSalesResponseVatEvidenceBuilder::captured_at)
    /// - [`issue_date`](InvoicesLockSalesResponseVatEvidenceBuilder::issue_date)
    /// - [`scheme`](InvoicesLockSalesResponseVatEvidenceBuilder::scheme)
    /// - [`partner`](InvoicesLockSalesResponseVatEvidenceBuilder::partner)
    /// - [`location`](InvoicesLockSalesResponseVatEvidenceBuilder::location)
    /// - [`rates`](InvoicesLockSalesResponseVatEvidenceBuilder::rates)
    pub fn build(self) -> Result<InvoicesLockSalesResponseVatEvidence, BuildError> {
        Ok(InvoicesLockSalesResponseVatEvidence {
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

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesCreateSalesResponseVatEvidence {
    #[serde(rename = "capturedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub captured_at: DateTime<FixedOffset>,
    #[serde(rename = "issueDate")]
    #[serde(default)]
    pub issue_date: NaiveDate,
    #[serde(default)]
    pub scheme: InvoicesCreateSalesResponseVatEvidenceScheme,
    #[serde(default)]
    pub partner: InvoicesCreateSalesResponseVatEvidencePartner,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vies: Option<InvoicesCreateSalesResponseVatEvidenceVies>,
    #[serde(default)]
    pub location: InvoicesCreateSalesResponseVatEvidenceLocation,
    #[serde(rename = "rateTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_table: Option<InvoicesCreateSalesResponseVatEvidenceRateTable>,
    #[serde(default)]
    pub rates: Vec<InvoicesCreateSalesResponseVatEvidenceRatesItem>,
}

impl InvoicesCreateSalesResponseVatEvidence {
    pub fn builder() -> InvoicesCreateSalesResponseVatEvidenceBuilder {
        <InvoicesCreateSalesResponseVatEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesCreateSalesResponseVatEvidenceBuilder {
    captured_at: Option<DateTime<FixedOffset>>,
    issue_date: Option<NaiveDate>,
    scheme: Option<InvoicesCreateSalesResponseVatEvidenceScheme>,
    partner: Option<InvoicesCreateSalesResponseVatEvidencePartner>,
    vies: Option<InvoicesCreateSalesResponseVatEvidenceVies>,
    location: Option<InvoicesCreateSalesResponseVatEvidenceLocation>,
    rate_table: Option<InvoicesCreateSalesResponseVatEvidenceRateTable>,
    rates: Option<Vec<InvoicesCreateSalesResponseVatEvidenceRatesItem>>,
}

impl InvoicesCreateSalesResponseVatEvidenceBuilder {
    pub fn captured_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.captured_at = Some(value);
        self
    }

    pub fn issue_date(mut self, value: NaiveDate) -> Self {
        self.issue_date = Some(value);
        self
    }

    pub fn scheme(mut self, value: InvoicesCreateSalesResponseVatEvidenceScheme) -> Self {
        self.scheme = Some(value);
        self
    }

    pub fn partner(mut self, value: InvoicesCreateSalesResponseVatEvidencePartner) -> Self {
        self.partner = Some(value);
        self
    }

    pub fn vies(mut self, value: InvoicesCreateSalesResponseVatEvidenceVies) -> Self {
        self.vies = Some(value);
        self
    }

    pub fn location(mut self, value: InvoicesCreateSalesResponseVatEvidenceLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn rate_table(mut self, value: InvoicesCreateSalesResponseVatEvidenceRateTable) -> Self {
        self.rate_table = Some(value);
        self
    }

    pub fn rates(mut self, value: Vec<InvoicesCreateSalesResponseVatEvidenceRatesItem>) -> Self {
        self.rates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoicesCreateSalesResponseVatEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`captured_at`](InvoicesCreateSalesResponseVatEvidenceBuilder::captured_at)
    /// - [`issue_date`](InvoicesCreateSalesResponseVatEvidenceBuilder::issue_date)
    /// - [`scheme`](InvoicesCreateSalesResponseVatEvidenceBuilder::scheme)
    /// - [`partner`](InvoicesCreateSalesResponseVatEvidenceBuilder::partner)
    /// - [`location`](InvoicesCreateSalesResponseVatEvidenceBuilder::location)
    /// - [`rates`](InvoicesCreateSalesResponseVatEvidenceBuilder::rates)
    pub fn build(self) -> Result<InvoicesCreateSalesResponseVatEvidence, BuildError> {
        Ok(InvoicesCreateSalesResponseVatEvidence {
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

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesGetSalesResponseVatEvidence {
    #[serde(rename = "capturedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub captured_at: DateTime<FixedOffset>,
    #[serde(rename = "issueDate")]
    #[serde(default)]
    pub issue_date: NaiveDate,
    #[serde(default)]
    pub scheme: InvoicesGetSalesResponseVatEvidenceScheme,
    #[serde(default)]
    pub partner: InvoicesGetSalesResponseVatEvidencePartner,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vies: Option<InvoicesGetSalesResponseVatEvidenceVies>,
    #[serde(default)]
    pub location: InvoicesGetSalesResponseVatEvidenceLocation,
    #[serde(rename = "rateTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_table: Option<InvoicesGetSalesResponseVatEvidenceRateTable>,
    #[serde(default)]
    pub rates: Vec<InvoicesGetSalesResponseVatEvidenceRatesItem>,
}

impl InvoicesGetSalesResponseVatEvidence {
    pub fn builder() -> InvoicesGetSalesResponseVatEvidenceBuilder {
        <InvoicesGetSalesResponseVatEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesGetSalesResponseVatEvidenceBuilder {
    captured_at: Option<DateTime<FixedOffset>>,
    issue_date: Option<NaiveDate>,
    scheme: Option<InvoicesGetSalesResponseVatEvidenceScheme>,
    partner: Option<InvoicesGetSalesResponseVatEvidencePartner>,
    vies: Option<InvoicesGetSalesResponseVatEvidenceVies>,
    location: Option<InvoicesGetSalesResponseVatEvidenceLocation>,
    rate_table: Option<InvoicesGetSalesResponseVatEvidenceRateTable>,
    rates: Option<Vec<InvoicesGetSalesResponseVatEvidenceRatesItem>>,
}

impl InvoicesGetSalesResponseVatEvidenceBuilder {
    pub fn captured_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.captured_at = Some(value);
        self
    }

    pub fn issue_date(mut self, value: NaiveDate) -> Self {
        self.issue_date = Some(value);
        self
    }

    pub fn scheme(mut self, value: InvoicesGetSalesResponseVatEvidenceScheme) -> Self {
        self.scheme = Some(value);
        self
    }

    pub fn partner(mut self, value: InvoicesGetSalesResponseVatEvidencePartner) -> Self {
        self.partner = Some(value);
        self
    }

    pub fn vies(mut self, value: InvoicesGetSalesResponseVatEvidenceVies) -> Self {
        self.vies = Some(value);
        self
    }

    pub fn location(mut self, value: InvoicesGetSalesResponseVatEvidenceLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn rate_table(mut self, value: InvoicesGetSalesResponseVatEvidenceRateTable) -> Self {
        self.rate_table = Some(value);
        self
    }

    pub fn rates(mut self, value: Vec<InvoicesGetSalesResponseVatEvidenceRatesItem>) -> Self {
        self.rates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoicesGetSalesResponseVatEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`captured_at`](InvoicesGetSalesResponseVatEvidenceBuilder::captured_at)
    /// - [`issue_date`](InvoicesGetSalesResponseVatEvidenceBuilder::issue_date)
    /// - [`scheme`](InvoicesGetSalesResponseVatEvidenceBuilder::scheme)
    /// - [`partner`](InvoicesGetSalesResponseVatEvidenceBuilder::partner)
    /// - [`location`](InvoicesGetSalesResponseVatEvidenceBuilder::location)
    /// - [`rates`](InvoicesGetSalesResponseVatEvidenceBuilder::rates)
    pub fn build(self) -> Result<InvoicesGetSalesResponseVatEvidence, BuildError> {
        Ok(InvoicesGetSalesResponseVatEvidence {
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

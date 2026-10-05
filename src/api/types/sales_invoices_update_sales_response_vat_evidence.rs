pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesUpdateSalesResponseVatEvidence {
    #[serde(rename = "capturedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub captured_at: DateTime<FixedOffset>,
    #[serde(rename = "issueDate")]
    #[serde(default)]
    pub issue_date: NaiveDate,
    #[serde(default)]
    pub scheme: InvoicesUpdateSalesResponseVatEvidenceScheme,
    #[serde(default)]
    pub partner: InvoicesUpdateSalesResponseVatEvidencePartner,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vies: Option<InvoicesUpdateSalesResponseVatEvidenceVies>,
    #[serde(default)]
    pub location: InvoicesUpdateSalesResponseVatEvidenceLocation,
    #[serde(rename = "rateTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_table: Option<InvoicesUpdateSalesResponseVatEvidenceRateTable>,
    #[serde(default)]
    pub rates: Vec<InvoicesUpdateSalesResponseVatEvidenceRatesItem>,
}

impl InvoicesUpdateSalesResponseVatEvidence {
    pub fn builder() -> InvoicesUpdateSalesResponseVatEvidenceBuilder {
        <InvoicesUpdateSalesResponseVatEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesUpdateSalesResponseVatEvidenceBuilder {
    captured_at: Option<DateTime<FixedOffset>>,
    issue_date: Option<NaiveDate>,
    scheme: Option<InvoicesUpdateSalesResponseVatEvidenceScheme>,
    partner: Option<InvoicesUpdateSalesResponseVatEvidencePartner>,
    vies: Option<InvoicesUpdateSalesResponseVatEvidenceVies>,
    location: Option<InvoicesUpdateSalesResponseVatEvidenceLocation>,
    rate_table: Option<InvoicesUpdateSalesResponseVatEvidenceRateTable>,
    rates: Option<Vec<InvoicesUpdateSalesResponseVatEvidenceRatesItem>>,
}

impl InvoicesUpdateSalesResponseVatEvidenceBuilder {
    pub fn captured_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.captured_at = Some(value);
        self
    }

    pub fn issue_date(mut self, value: NaiveDate) -> Self {
        self.issue_date = Some(value);
        self
    }

    pub fn scheme(mut self, value: InvoicesUpdateSalesResponseVatEvidenceScheme) -> Self {
        self.scheme = Some(value);
        self
    }

    pub fn partner(mut self, value: InvoicesUpdateSalesResponseVatEvidencePartner) -> Self {
        self.partner = Some(value);
        self
    }

    pub fn vies(mut self, value: InvoicesUpdateSalesResponseVatEvidenceVies) -> Self {
        self.vies = Some(value);
        self
    }

    pub fn location(mut self, value: InvoicesUpdateSalesResponseVatEvidenceLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn rate_table(mut self, value: InvoicesUpdateSalesResponseVatEvidenceRateTable) -> Self {
        self.rate_table = Some(value);
        self
    }

    pub fn rates(mut self, value: Vec<InvoicesUpdateSalesResponseVatEvidenceRatesItem>) -> Self {
        self.rates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoicesUpdateSalesResponseVatEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`captured_at`](InvoicesUpdateSalesResponseVatEvidenceBuilder::captured_at)
    /// - [`issue_date`](InvoicesUpdateSalesResponseVatEvidenceBuilder::issue_date)
    /// - [`scheme`](InvoicesUpdateSalesResponseVatEvidenceBuilder::scheme)
    /// - [`partner`](InvoicesUpdateSalesResponseVatEvidenceBuilder::partner)
    /// - [`location`](InvoicesUpdateSalesResponseVatEvidenceBuilder::location)
    /// - [`rates`](InvoicesUpdateSalesResponseVatEvidenceBuilder::rates)
    pub fn build(self) -> Result<InvoicesUpdateSalesResponseVatEvidence, BuildError> {
        Ok(InvoicesUpdateSalesResponseVatEvidence {
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

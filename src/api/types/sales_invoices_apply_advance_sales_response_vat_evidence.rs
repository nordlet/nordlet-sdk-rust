pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesApplyAdvanceSalesResponseVatEvidence {
    #[serde(rename = "capturedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub captured_at: DateTime<FixedOffset>,
    #[serde(rename = "issueDate")]
    #[serde(default)]
    pub issue_date: NaiveDate,
    #[serde(default)]
    pub scheme: InvoicesApplyAdvanceSalesResponseVatEvidenceScheme,
    #[serde(default)]
    pub partner: InvoicesApplyAdvanceSalesResponseVatEvidencePartner,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vies: Option<InvoicesApplyAdvanceSalesResponseVatEvidenceVies>,
    #[serde(default)]
    pub location: InvoicesApplyAdvanceSalesResponseVatEvidenceLocation,
    #[serde(rename = "rateTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_table: Option<InvoicesApplyAdvanceSalesResponseVatEvidenceRateTable>,
    #[serde(default)]
    pub rates: Vec<InvoicesApplyAdvanceSalesResponseVatEvidenceRatesItem>,
}

impl InvoicesApplyAdvanceSalesResponseVatEvidence {
    pub fn builder() -> InvoicesApplyAdvanceSalesResponseVatEvidenceBuilder {
        <InvoicesApplyAdvanceSalesResponseVatEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesApplyAdvanceSalesResponseVatEvidenceBuilder {
    captured_at: Option<DateTime<FixedOffset>>,
    issue_date: Option<NaiveDate>,
    scheme: Option<InvoicesApplyAdvanceSalesResponseVatEvidenceScheme>,
    partner: Option<InvoicesApplyAdvanceSalesResponseVatEvidencePartner>,
    vies: Option<InvoicesApplyAdvanceSalesResponseVatEvidenceVies>,
    location: Option<InvoicesApplyAdvanceSalesResponseVatEvidenceLocation>,
    rate_table: Option<InvoicesApplyAdvanceSalesResponseVatEvidenceRateTable>,
    rates: Option<Vec<InvoicesApplyAdvanceSalesResponseVatEvidenceRatesItem>>,
}

impl InvoicesApplyAdvanceSalesResponseVatEvidenceBuilder {
    pub fn captured_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.captured_at = Some(value);
        self
    }

    pub fn issue_date(mut self, value: NaiveDate) -> Self {
        self.issue_date = Some(value);
        self
    }

    pub fn scheme(mut self, value: InvoicesApplyAdvanceSalesResponseVatEvidenceScheme) -> Self {
        self.scheme = Some(value);
        self
    }

    pub fn partner(mut self, value: InvoicesApplyAdvanceSalesResponseVatEvidencePartner) -> Self {
        self.partner = Some(value);
        self
    }

    pub fn vies(mut self, value: InvoicesApplyAdvanceSalesResponseVatEvidenceVies) -> Self {
        self.vies = Some(value);
        self
    }

    pub fn location(mut self, value: InvoicesApplyAdvanceSalesResponseVatEvidenceLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn rate_table(
        mut self,
        value: InvoicesApplyAdvanceSalesResponseVatEvidenceRateTable,
    ) -> Self {
        self.rate_table = Some(value);
        self
    }

    pub fn rates(
        mut self,
        value: Vec<InvoicesApplyAdvanceSalesResponseVatEvidenceRatesItem>,
    ) -> Self {
        self.rates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoicesApplyAdvanceSalesResponseVatEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`captured_at`](InvoicesApplyAdvanceSalesResponseVatEvidenceBuilder::captured_at)
    /// - [`issue_date`](InvoicesApplyAdvanceSalesResponseVatEvidenceBuilder::issue_date)
    /// - [`scheme`](InvoicesApplyAdvanceSalesResponseVatEvidenceBuilder::scheme)
    /// - [`partner`](InvoicesApplyAdvanceSalesResponseVatEvidenceBuilder::partner)
    /// - [`location`](InvoicesApplyAdvanceSalesResponseVatEvidenceBuilder::location)
    /// - [`rates`](InvoicesApplyAdvanceSalesResponseVatEvidenceBuilder::rates)
    pub fn build(self) -> Result<InvoicesApplyAdvanceSalesResponseVatEvidence, BuildError> {
        Ok(InvoicesApplyAdvanceSalesResponseVatEvidence {
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

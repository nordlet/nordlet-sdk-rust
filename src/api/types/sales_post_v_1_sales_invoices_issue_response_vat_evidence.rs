pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesIssueResponseVatEvidence {
    #[serde(rename = "capturedAt")]
    #[serde(default)]
    pub captured_at: String,
    #[serde(rename = "issueDate")]
    #[serde(default)]
    pub issue_date: String,
    #[serde(default)]
    pub scheme: PostV1SalesInvoicesIssueResponseVatEvidenceScheme,
    #[serde(default)]
    pub partner: PostV1SalesInvoicesIssueResponseVatEvidencePartner,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vies: Option<PostV1SalesInvoicesIssueResponseVatEvidenceVies>,
    #[serde(default)]
    pub location: PostV1SalesInvoicesIssueResponseVatEvidenceLocation,
    #[serde(rename = "rateTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_table: Option<PostV1SalesInvoicesIssueResponseVatEvidenceRateTable>,
    #[serde(default)]
    pub rates: Vec<PostV1SalesInvoicesIssueResponseVatEvidenceRatesItem>,
}

impl PostV1SalesInvoicesIssueResponseVatEvidence {
    pub fn builder() -> PostV1SalesInvoicesIssueResponseVatEvidenceBuilder {
        <PostV1SalesInvoicesIssueResponseVatEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesIssueResponseVatEvidenceBuilder {
    captured_at: Option<String>,
    issue_date: Option<String>,
    scheme: Option<PostV1SalesInvoicesIssueResponseVatEvidenceScheme>,
    partner: Option<PostV1SalesInvoicesIssueResponseVatEvidencePartner>,
    vies: Option<PostV1SalesInvoicesIssueResponseVatEvidenceVies>,
    location: Option<PostV1SalesInvoicesIssueResponseVatEvidenceLocation>,
    rate_table: Option<PostV1SalesInvoicesIssueResponseVatEvidenceRateTable>,
    rates: Option<Vec<PostV1SalesInvoicesIssueResponseVatEvidenceRatesItem>>,
}

impl PostV1SalesInvoicesIssueResponseVatEvidenceBuilder {
    pub fn captured_at(mut self, value: impl Into<String>) -> Self {
        self.captured_at = Some(value.into());
        self
    }

    pub fn issue_date(mut self, value: impl Into<String>) -> Self {
        self.issue_date = Some(value.into());
        self
    }

    pub fn scheme(mut self, value: PostV1SalesInvoicesIssueResponseVatEvidenceScheme) -> Self {
        self.scheme = Some(value);
        self
    }

    pub fn partner(mut self, value: PostV1SalesInvoicesIssueResponseVatEvidencePartner) -> Self {
        self.partner = Some(value);
        self
    }

    pub fn vies(mut self, value: PostV1SalesInvoicesIssueResponseVatEvidenceVies) -> Self {
        self.vies = Some(value);
        self
    }

    pub fn location(mut self, value: PostV1SalesInvoicesIssueResponseVatEvidenceLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn rate_table(
        mut self,
        value: PostV1SalesInvoicesIssueResponseVatEvidenceRateTable,
    ) -> Self {
        self.rate_table = Some(value);
        self
    }

    pub fn rates(
        mut self,
        value: Vec<PostV1SalesInvoicesIssueResponseVatEvidenceRatesItem>,
    ) -> Self {
        self.rates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesIssueResponseVatEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`captured_at`](PostV1SalesInvoicesIssueResponseVatEvidenceBuilder::captured_at)
    /// - [`issue_date`](PostV1SalesInvoicesIssueResponseVatEvidenceBuilder::issue_date)
    /// - [`scheme`](PostV1SalesInvoicesIssueResponseVatEvidenceBuilder::scheme)
    /// - [`partner`](PostV1SalesInvoicesIssueResponseVatEvidenceBuilder::partner)
    /// - [`location`](PostV1SalesInvoicesIssueResponseVatEvidenceBuilder::location)
    /// - [`rates`](PostV1SalesInvoicesIssueResponseVatEvidenceBuilder::rates)
    pub fn build(self) -> Result<PostV1SalesInvoicesIssueResponseVatEvidence, BuildError> {
        Ok(PostV1SalesInvoicesIssueResponseVatEvidence {
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

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesApplyAdvanceResponseVatEvidence {
    #[serde(rename = "capturedAt")]
    #[serde(default)]
    pub captured_at: String,
    #[serde(rename = "issueDate")]
    #[serde(default)]
    pub issue_date: String,
    #[serde(default)]
    pub scheme: PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceScheme,
    #[serde(default)]
    pub partner: PostV1SalesInvoicesApplyAdvanceResponseVatEvidencePartner,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vies: Option<PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceVies>,
    #[serde(default)]
    pub location: PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceLocation,
    #[serde(rename = "rateTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_table: Option<PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceRateTable>,
    #[serde(default)]
    pub rates: Vec<PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceRatesItem>,
}

impl PostV1SalesInvoicesApplyAdvanceResponseVatEvidence {
    pub fn builder() -> PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceBuilder {
        <PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceBuilder {
    captured_at: Option<String>,
    issue_date: Option<String>,
    scheme: Option<PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceScheme>,
    partner: Option<PostV1SalesInvoicesApplyAdvanceResponseVatEvidencePartner>,
    vies: Option<PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceVies>,
    location: Option<PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceLocation>,
    rate_table: Option<PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceRateTable>,
    rates: Option<Vec<PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceRatesItem>>,
}

impl PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceBuilder {
    pub fn captured_at(mut self, value: impl Into<String>) -> Self {
        self.captured_at = Some(value.into());
        self
    }

    pub fn issue_date(mut self, value: impl Into<String>) -> Self {
        self.issue_date = Some(value.into());
        self
    }

    pub fn scheme(
        mut self,
        value: PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceScheme,
    ) -> Self {
        self.scheme = Some(value);
        self
    }

    pub fn partner(
        mut self,
        value: PostV1SalesInvoicesApplyAdvanceResponseVatEvidencePartner,
    ) -> Self {
        self.partner = Some(value);
        self
    }

    pub fn vies(mut self, value: PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceVies) -> Self {
        self.vies = Some(value);
        self
    }

    pub fn location(
        mut self,
        value: PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceLocation,
    ) -> Self {
        self.location = Some(value);
        self
    }

    pub fn rate_table(
        mut self,
        value: PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceRateTable,
    ) -> Self {
        self.rate_table = Some(value);
        self
    }

    pub fn rates(
        mut self,
        value: Vec<PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceRatesItem>,
    ) -> Self {
        self.rates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesApplyAdvanceResponseVatEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`captured_at`](PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceBuilder::captured_at)
    /// - [`issue_date`](PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceBuilder::issue_date)
    /// - [`scheme`](PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceBuilder::scheme)
    /// - [`partner`](PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceBuilder::partner)
    /// - [`location`](PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceBuilder::location)
    /// - [`rates`](PostV1SalesInvoicesApplyAdvanceResponseVatEvidenceBuilder::rates)
    pub fn build(self) -> Result<PostV1SalesInvoicesApplyAdvanceResponseVatEvidence, BuildError> {
        Ok(PostV1SalesInvoicesApplyAdvanceResponseVatEvidence {
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

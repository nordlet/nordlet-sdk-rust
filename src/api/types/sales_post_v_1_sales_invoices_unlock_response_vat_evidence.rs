pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesUnlockResponseVatEvidence {
    #[serde(rename = "capturedAt")]
    #[serde(default)]
    pub captured_at: String,
    #[serde(rename = "issueDate")]
    #[serde(default)]
    pub issue_date: String,
    #[serde(default)]
    pub scheme: PostV1SalesInvoicesUnlockResponseVatEvidenceScheme,
    #[serde(default)]
    pub partner: PostV1SalesInvoicesUnlockResponseVatEvidencePartner,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vies: Option<PostV1SalesInvoicesUnlockResponseVatEvidenceVies>,
    #[serde(default)]
    pub location: PostV1SalesInvoicesUnlockResponseVatEvidenceLocation,
    #[serde(rename = "rateTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_table: Option<PostV1SalesInvoicesUnlockResponseVatEvidenceRateTable>,
    #[serde(default)]
    pub rates: Vec<PostV1SalesInvoicesUnlockResponseVatEvidenceRatesItem>,
}

impl PostV1SalesInvoicesUnlockResponseVatEvidence {
    pub fn builder() -> PostV1SalesInvoicesUnlockResponseVatEvidenceBuilder {
        <PostV1SalesInvoicesUnlockResponseVatEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesUnlockResponseVatEvidenceBuilder {
    captured_at: Option<String>,
    issue_date: Option<String>,
    scheme: Option<PostV1SalesInvoicesUnlockResponseVatEvidenceScheme>,
    partner: Option<PostV1SalesInvoicesUnlockResponseVatEvidencePartner>,
    vies: Option<PostV1SalesInvoicesUnlockResponseVatEvidenceVies>,
    location: Option<PostV1SalesInvoicesUnlockResponseVatEvidenceLocation>,
    rate_table: Option<PostV1SalesInvoicesUnlockResponseVatEvidenceRateTable>,
    rates: Option<Vec<PostV1SalesInvoicesUnlockResponseVatEvidenceRatesItem>>,
}

impl PostV1SalesInvoicesUnlockResponseVatEvidenceBuilder {
    pub fn captured_at(mut self, value: impl Into<String>) -> Self {
        self.captured_at = Some(value.into());
        self
    }

    pub fn issue_date(mut self, value: impl Into<String>) -> Self {
        self.issue_date = Some(value.into());
        self
    }

    pub fn scheme(mut self, value: PostV1SalesInvoicesUnlockResponseVatEvidenceScheme) -> Self {
        self.scheme = Some(value);
        self
    }

    pub fn partner(mut self, value: PostV1SalesInvoicesUnlockResponseVatEvidencePartner) -> Self {
        self.partner = Some(value);
        self
    }

    pub fn vies(mut self, value: PostV1SalesInvoicesUnlockResponseVatEvidenceVies) -> Self {
        self.vies = Some(value);
        self
    }

    pub fn location(mut self, value: PostV1SalesInvoicesUnlockResponseVatEvidenceLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn rate_table(
        mut self,
        value: PostV1SalesInvoicesUnlockResponseVatEvidenceRateTable,
    ) -> Self {
        self.rate_table = Some(value);
        self
    }

    pub fn rates(
        mut self,
        value: Vec<PostV1SalesInvoicesUnlockResponseVatEvidenceRatesItem>,
    ) -> Self {
        self.rates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesUnlockResponseVatEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`captured_at`](PostV1SalesInvoicesUnlockResponseVatEvidenceBuilder::captured_at)
    /// - [`issue_date`](PostV1SalesInvoicesUnlockResponseVatEvidenceBuilder::issue_date)
    /// - [`scheme`](PostV1SalesInvoicesUnlockResponseVatEvidenceBuilder::scheme)
    /// - [`partner`](PostV1SalesInvoicesUnlockResponseVatEvidenceBuilder::partner)
    /// - [`location`](PostV1SalesInvoicesUnlockResponseVatEvidenceBuilder::location)
    /// - [`rates`](PostV1SalesInvoicesUnlockResponseVatEvidenceBuilder::rates)
    pub fn build(self) -> Result<PostV1SalesInvoicesUnlockResponseVatEvidence, BuildError> {
        Ok(PostV1SalesInvoicesUnlockResponseVatEvidence {
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

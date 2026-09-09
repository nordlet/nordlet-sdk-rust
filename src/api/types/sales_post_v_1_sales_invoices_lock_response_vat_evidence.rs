pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesLockResponseVatEvidence {
    #[serde(rename = "capturedAt")]
    #[serde(default)]
    pub captured_at: String,
    #[serde(rename = "issueDate")]
    #[serde(default)]
    pub issue_date: String,
    #[serde(default)]
    pub scheme: PostV1SalesInvoicesLockResponseVatEvidenceScheme,
    #[serde(default)]
    pub partner: PostV1SalesInvoicesLockResponseVatEvidencePartner,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vies: Option<PostV1SalesInvoicesLockResponseVatEvidenceVies>,
    #[serde(default)]
    pub location: PostV1SalesInvoicesLockResponseVatEvidenceLocation,
    #[serde(rename = "rateTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_table: Option<PostV1SalesInvoicesLockResponseVatEvidenceRateTable>,
    #[serde(default)]
    pub rates: Vec<PostV1SalesInvoicesLockResponseVatEvidenceRatesItem>,
}

impl PostV1SalesInvoicesLockResponseVatEvidence {
    pub fn builder() -> PostV1SalesInvoicesLockResponseVatEvidenceBuilder {
        <PostV1SalesInvoicesLockResponseVatEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesLockResponseVatEvidenceBuilder {
    captured_at: Option<String>,
    issue_date: Option<String>,
    scheme: Option<PostV1SalesInvoicesLockResponseVatEvidenceScheme>,
    partner: Option<PostV1SalesInvoicesLockResponseVatEvidencePartner>,
    vies: Option<PostV1SalesInvoicesLockResponseVatEvidenceVies>,
    location: Option<PostV1SalesInvoicesLockResponseVatEvidenceLocation>,
    rate_table: Option<PostV1SalesInvoicesLockResponseVatEvidenceRateTable>,
    rates: Option<Vec<PostV1SalesInvoicesLockResponseVatEvidenceRatesItem>>,
}

impl PostV1SalesInvoicesLockResponseVatEvidenceBuilder {
    pub fn captured_at(mut self, value: impl Into<String>) -> Self {
        self.captured_at = Some(value.into());
        self
    }

    pub fn issue_date(mut self, value: impl Into<String>) -> Self {
        self.issue_date = Some(value.into());
        self
    }

    pub fn scheme(mut self, value: PostV1SalesInvoicesLockResponseVatEvidenceScheme) -> Self {
        self.scheme = Some(value);
        self
    }

    pub fn partner(mut self, value: PostV1SalesInvoicesLockResponseVatEvidencePartner) -> Self {
        self.partner = Some(value);
        self
    }

    pub fn vies(mut self, value: PostV1SalesInvoicesLockResponseVatEvidenceVies) -> Self {
        self.vies = Some(value);
        self
    }

    pub fn location(mut self, value: PostV1SalesInvoicesLockResponseVatEvidenceLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn rate_table(
        mut self,
        value: PostV1SalesInvoicesLockResponseVatEvidenceRateTable,
    ) -> Self {
        self.rate_table = Some(value);
        self
    }

    pub fn rates(
        mut self,
        value: Vec<PostV1SalesInvoicesLockResponseVatEvidenceRatesItem>,
    ) -> Self {
        self.rates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesLockResponseVatEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`captured_at`](PostV1SalesInvoicesLockResponseVatEvidenceBuilder::captured_at)
    /// - [`issue_date`](PostV1SalesInvoicesLockResponseVatEvidenceBuilder::issue_date)
    /// - [`scheme`](PostV1SalesInvoicesLockResponseVatEvidenceBuilder::scheme)
    /// - [`partner`](PostV1SalesInvoicesLockResponseVatEvidenceBuilder::partner)
    /// - [`location`](PostV1SalesInvoicesLockResponseVatEvidenceBuilder::location)
    /// - [`rates`](PostV1SalesInvoicesLockResponseVatEvidenceBuilder::rates)
    pub fn build(self) -> Result<PostV1SalesInvoicesLockResponseVatEvidence, BuildError> {
        Ok(PostV1SalesInvoicesLockResponseVatEvidence {
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

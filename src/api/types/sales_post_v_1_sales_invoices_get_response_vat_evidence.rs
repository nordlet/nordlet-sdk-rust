pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesGetResponseVatEvidence {
    #[serde(rename = "capturedAt")]
    #[serde(default)]
    pub captured_at: String,
    #[serde(rename = "issueDate")]
    #[serde(default)]
    pub issue_date: String,
    #[serde(default)]
    pub scheme: PostV1SalesInvoicesGetResponseVatEvidenceScheme,
    #[serde(default)]
    pub partner: PostV1SalesInvoicesGetResponseVatEvidencePartner,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vies: Option<PostV1SalesInvoicesGetResponseVatEvidenceVies>,
    #[serde(default)]
    pub location: PostV1SalesInvoicesGetResponseVatEvidenceLocation,
    #[serde(rename = "rateTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_table: Option<PostV1SalesInvoicesGetResponseVatEvidenceRateTable>,
    #[serde(default)]
    pub rates: Vec<PostV1SalesInvoicesGetResponseVatEvidenceRatesItem>,
}

impl PostV1SalesInvoicesGetResponseVatEvidence {
    pub fn builder() -> PostV1SalesInvoicesGetResponseVatEvidenceBuilder {
        <PostV1SalesInvoicesGetResponseVatEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesGetResponseVatEvidenceBuilder {
    captured_at: Option<String>,
    issue_date: Option<String>,
    scheme: Option<PostV1SalesInvoicesGetResponseVatEvidenceScheme>,
    partner: Option<PostV1SalesInvoicesGetResponseVatEvidencePartner>,
    vies: Option<PostV1SalesInvoicesGetResponseVatEvidenceVies>,
    location: Option<PostV1SalesInvoicesGetResponseVatEvidenceLocation>,
    rate_table: Option<PostV1SalesInvoicesGetResponseVatEvidenceRateTable>,
    rates: Option<Vec<PostV1SalesInvoicesGetResponseVatEvidenceRatesItem>>,
}

impl PostV1SalesInvoicesGetResponseVatEvidenceBuilder {
    pub fn captured_at(mut self, value: impl Into<String>) -> Self {
        self.captured_at = Some(value.into());
        self
    }

    pub fn issue_date(mut self, value: impl Into<String>) -> Self {
        self.issue_date = Some(value.into());
        self
    }

    pub fn scheme(mut self, value: PostV1SalesInvoicesGetResponseVatEvidenceScheme) -> Self {
        self.scheme = Some(value);
        self
    }

    pub fn partner(mut self, value: PostV1SalesInvoicesGetResponseVatEvidencePartner) -> Self {
        self.partner = Some(value);
        self
    }

    pub fn vies(mut self, value: PostV1SalesInvoicesGetResponseVatEvidenceVies) -> Self {
        self.vies = Some(value);
        self
    }

    pub fn location(mut self, value: PostV1SalesInvoicesGetResponseVatEvidenceLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn rate_table(mut self, value: PostV1SalesInvoicesGetResponseVatEvidenceRateTable) -> Self {
        self.rate_table = Some(value);
        self
    }

    pub fn rates(mut self, value: Vec<PostV1SalesInvoicesGetResponseVatEvidenceRatesItem>) -> Self {
        self.rates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesGetResponseVatEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`captured_at`](PostV1SalesInvoicesGetResponseVatEvidenceBuilder::captured_at)
    /// - [`issue_date`](PostV1SalesInvoicesGetResponseVatEvidenceBuilder::issue_date)
    /// - [`scheme`](PostV1SalesInvoicesGetResponseVatEvidenceBuilder::scheme)
    /// - [`partner`](PostV1SalesInvoicesGetResponseVatEvidenceBuilder::partner)
    /// - [`location`](PostV1SalesInvoicesGetResponseVatEvidenceBuilder::location)
    /// - [`rates`](PostV1SalesInvoicesGetResponseVatEvidenceBuilder::rates)
    pub fn build(self) -> Result<PostV1SalesInvoicesGetResponseVatEvidence, BuildError> {
        Ok(PostV1SalesInvoicesGetResponseVatEvidence {
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

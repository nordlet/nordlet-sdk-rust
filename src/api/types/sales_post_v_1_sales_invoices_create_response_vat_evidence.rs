pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesCreateResponseVatEvidence {
    #[serde(rename = "capturedAt")]
    #[serde(default)]
    pub captured_at: String,
    #[serde(rename = "issueDate")]
    #[serde(default)]
    pub issue_date: String,
    #[serde(default)]
    pub scheme: PostV1SalesInvoicesCreateResponseVatEvidenceScheme,
    #[serde(default)]
    pub partner: PostV1SalesInvoicesCreateResponseVatEvidencePartner,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vies: Option<PostV1SalesInvoicesCreateResponseVatEvidenceVies>,
    #[serde(default)]
    pub location: PostV1SalesInvoicesCreateResponseVatEvidenceLocation,
    #[serde(rename = "rateTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_table: Option<PostV1SalesInvoicesCreateResponseVatEvidenceRateTable>,
    #[serde(default)]
    pub rates: Vec<PostV1SalesInvoicesCreateResponseVatEvidenceRatesItem>,
}

impl PostV1SalesInvoicesCreateResponseVatEvidence {
    pub fn builder() -> PostV1SalesInvoicesCreateResponseVatEvidenceBuilder {
        <PostV1SalesInvoicesCreateResponseVatEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesCreateResponseVatEvidenceBuilder {
    captured_at: Option<String>,
    issue_date: Option<String>,
    scheme: Option<PostV1SalesInvoicesCreateResponseVatEvidenceScheme>,
    partner: Option<PostV1SalesInvoicesCreateResponseVatEvidencePartner>,
    vies: Option<PostV1SalesInvoicesCreateResponseVatEvidenceVies>,
    location: Option<PostV1SalesInvoicesCreateResponseVatEvidenceLocation>,
    rate_table: Option<PostV1SalesInvoicesCreateResponseVatEvidenceRateTable>,
    rates: Option<Vec<PostV1SalesInvoicesCreateResponseVatEvidenceRatesItem>>,
}

impl PostV1SalesInvoicesCreateResponseVatEvidenceBuilder {
    pub fn captured_at(mut self, value: impl Into<String>) -> Self {
        self.captured_at = Some(value.into());
        self
    }

    pub fn issue_date(mut self, value: impl Into<String>) -> Self {
        self.issue_date = Some(value.into());
        self
    }

    pub fn scheme(mut self, value: PostV1SalesInvoicesCreateResponseVatEvidenceScheme) -> Self {
        self.scheme = Some(value);
        self
    }

    pub fn partner(mut self, value: PostV1SalesInvoicesCreateResponseVatEvidencePartner) -> Self {
        self.partner = Some(value);
        self
    }

    pub fn vies(mut self, value: PostV1SalesInvoicesCreateResponseVatEvidenceVies) -> Self {
        self.vies = Some(value);
        self
    }

    pub fn location(mut self, value: PostV1SalesInvoicesCreateResponseVatEvidenceLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn rate_table(
        mut self,
        value: PostV1SalesInvoicesCreateResponseVatEvidenceRateTable,
    ) -> Self {
        self.rate_table = Some(value);
        self
    }

    pub fn rates(
        mut self,
        value: Vec<PostV1SalesInvoicesCreateResponseVatEvidenceRatesItem>,
    ) -> Self {
        self.rates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesCreateResponseVatEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`captured_at`](PostV1SalesInvoicesCreateResponseVatEvidenceBuilder::captured_at)
    /// - [`issue_date`](PostV1SalesInvoicesCreateResponseVatEvidenceBuilder::issue_date)
    /// - [`scheme`](PostV1SalesInvoicesCreateResponseVatEvidenceBuilder::scheme)
    /// - [`partner`](PostV1SalesInvoicesCreateResponseVatEvidenceBuilder::partner)
    /// - [`location`](PostV1SalesInvoicesCreateResponseVatEvidenceBuilder::location)
    /// - [`rates`](PostV1SalesInvoicesCreateResponseVatEvidenceBuilder::rates)
    pub fn build(self) -> Result<PostV1SalesInvoicesCreateResponseVatEvidence, BuildError> {
        Ok(PostV1SalesInvoicesCreateResponseVatEvidence {
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

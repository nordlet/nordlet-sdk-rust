pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesUpdateResponseVatEvidence {
    #[serde(rename = "capturedAt")]
    #[serde(default)]
    pub captured_at: String,
    #[serde(rename = "issueDate")]
    #[serde(default)]
    pub issue_date: String,
    #[serde(default)]
    pub scheme: PostV1SalesInvoicesUpdateResponseVatEvidenceScheme,
    #[serde(default)]
    pub partner: PostV1SalesInvoicesUpdateResponseVatEvidencePartner,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vies: Option<PostV1SalesInvoicesUpdateResponseVatEvidenceVies>,
    #[serde(default)]
    pub location: PostV1SalesInvoicesUpdateResponseVatEvidenceLocation,
    #[serde(rename = "rateTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_table: Option<PostV1SalesInvoicesUpdateResponseVatEvidenceRateTable>,
    #[serde(default)]
    pub rates: Vec<PostV1SalesInvoicesUpdateResponseVatEvidenceRatesItem>,
}

impl PostV1SalesInvoicesUpdateResponseVatEvidence {
    pub fn builder() -> PostV1SalesInvoicesUpdateResponseVatEvidenceBuilder {
        <PostV1SalesInvoicesUpdateResponseVatEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesUpdateResponseVatEvidenceBuilder {
    captured_at: Option<String>,
    issue_date: Option<String>,
    scheme: Option<PostV1SalesInvoicesUpdateResponseVatEvidenceScheme>,
    partner: Option<PostV1SalesInvoicesUpdateResponseVatEvidencePartner>,
    vies: Option<PostV1SalesInvoicesUpdateResponseVatEvidenceVies>,
    location: Option<PostV1SalesInvoicesUpdateResponseVatEvidenceLocation>,
    rate_table: Option<PostV1SalesInvoicesUpdateResponseVatEvidenceRateTable>,
    rates: Option<Vec<PostV1SalesInvoicesUpdateResponseVatEvidenceRatesItem>>,
}

impl PostV1SalesInvoicesUpdateResponseVatEvidenceBuilder {
    pub fn captured_at(mut self, value: impl Into<String>) -> Self {
        self.captured_at = Some(value.into());
        self
    }

    pub fn issue_date(mut self, value: impl Into<String>) -> Self {
        self.issue_date = Some(value.into());
        self
    }

    pub fn scheme(mut self, value: PostV1SalesInvoicesUpdateResponseVatEvidenceScheme) -> Self {
        self.scheme = Some(value);
        self
    }

    pub fn partner(mut self, value: PostV1SalesInvoicesUpdateResponseVatEvidencePartner) -> Self {
        self.partner = Some(value);
        self
    }

    pub fn vies(mut self, value: PostV1SalesInvoicesUpdateResponseVatEvidenceVies) -> Self {
        self.vies = Some(value);
        self
    }

    pub fn location(mut self, value: PostV1SalesInvoicesUpdateResponseVatEvidenceLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn rate_table(
        mut self,
        value: PostV1SalesInvoicesUpdateResponseVatEvidenceRateTable,
    ) -> Self {
        self.rate_table = Some(value);
        self
    }

    pub fn rates(
        mut self,
        value: Vec<PostV1SalesInvoicesUpdateResponseVatEvidenceRatesItem>,
    ) -> Self {
        self.rates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesUpdateResponseVatEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`captured_at`](PostV1SalesInvoicesUpdateResponseVatEvidenceBuilder::captured_at)
    /// - [`issue_date`](PostV1SalesInvoicesUpdateResponseVatEvidenceBuilder::issue_date)
    /// - [`scheme`](PostV1SalesInvoicesUpdateResponseVatEvidenceBuilder::scheme)
    /// - [`partner`](PostV1SalesInvoicesUpdateResponseVatEvidenceBuilder::partner)
    /// - [`location`](PostV1SalesInvoicesUpdateResponseVatEvidenceBuilder::location)
    /// - [`rates`](PostV1SalesInvoicesUpdateResponseVatEvidenceBuilder::rates)
    pub fn build(self) -> Result<PostV1SalesInvoicesUpdateResponseVatEvidence, BuildError> {
        Ok(PostV1SalesInvoicesUpdateResponseVatEvidence {
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

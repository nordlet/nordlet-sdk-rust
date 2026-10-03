pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1DeclarationsIeCt1GenerateResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "periodStart")]
    #[serde(default)]
    pub period_start: String,
    #[serde(rename = "periodEnd")]
    #[serde(default)]
    pub period_end: String,
    #[serde(rename = "taxRegNumber")]
    #[serde(default)]
    pub tax_reg_number: String,
    #[serde(default)]
    pub ct1: PostV1DeclarationsIeCt1GenerateResponseCt1,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<PostV1DeclarationsIeCt1GenerateResponseAccounts>,
    #[serde(rename = "accountsBlocking")]
    #[serde(default)]
    pub accounts_blocking: Vec<String>,
    #[serde(rename = "ixbrlMandatory")]
    #[serde(default)]
    pub ixbrl_mandatory: bool,
    #[serde(default)]
    pub criteria: PostV1DeclarationsIeCt1GenerateResponseCriteria,
    #[serde(default)]
    pub fields: Vec<PostV1DeclarationsIeCt1GenerateResponseFieldsItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl PostV1DeclarationsIeCt1GenerateResponse {
    pub fn builder() -> PostV1DeclarationsIeCt1GenerateResponseBuilder {
        <PostV1DeclarationsIeCt1GenerateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsIeCt1GenerateResponseBuilder {
    year: Option<i64>,
    period_start: Option<String>,
    period_end: Option<String>,
    tax_reg_number: Option<String>,
    ct1: Option<PostV1DeclarationsIeCt1GenerateResponseCt1>,
    accounts: Option<PostV1DeclarationsIeCt1GenerateResponseAccounts>,
    accounts_blocking: Option<Vec<String>>,
    ixbrl_mandatory: Option<bool>,
    criteria: Option<PostV1DeclarationsIeCt1GenerateResponseCriteria>,
    fields: Option<Vec<PostV1DeclarationsIeCt1GenerateResponseFieldsItem>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
}

impl PostV1DeclarationsIeCt1GenerateResponseBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn period_start(mut self, value: impl Into<String>) -> Self {
        self.period_start = Some(value.into());
        self
    }

    pub fn period_end(mut self, value: impl Into<String>) -> Self {
        self.period_end = Some(value.into());
        self
    }

    pub fn tax_reg_number(mut self, value: impl Into<String>) -> Self {
        self.tax_reg_number = Some(value.into());
        self
    }

    pub fn ct1(mut self, value: PostV1DeclarationsIeCt1GenerateResponseCt1) -> Self {
        self.ct1 = Some(value);
        self
    }

    pub fn accounts(mut self, value: PostV1DeclarationsIeCt1GenerateResponseAccounts) -> Self {
        self.accounts = Some(value);
        self
    }

    pub fn accounts_blocking(mut self, value: Vec<String>) -> Self {
        self.accounts_blocking = Some(value);
        self
    }

    pub fn ixbrl_mandatory(mut self, value: bool) -> Self {
        self.ixbrl_mandatory = Some(value);
        self
    }

    pub fn criteria(mut self, value: PostV1DeclarationsIeCt1GenerateResponseCriteria) -> Self {
        self.criteria = Some(value);
        self
    }

    pub fn fields(mut self, value: Vec<PostV1DeclarationsIeCt1GenerateResponseFieldsItem>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    pub fn notes(mut self, value: Vec<String>) -> Self {
        self.notes = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsIeCt1GenerateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsIeCt1GenerateResponseBuilder::year)
    /// - [`period_start`](PostV1DeclarationsIeCt1GenerateResponseBuilder::period_start)
    /// - [`period_end`](PostV1DeclarationsIeCt1GenerateResponseBuilder::period_end)
    /// - [`tax_reg_number`](PostV1DeclarationsIeCt1GenerateResponseBuilder::tax_reg_number)
    /// - [`ct1`](PostV1DeclarationsIeCt1GenerateResponseBuilder::ct1)
    /// - [`accounts_blocking`](PostV1DeclarationsIeCt1GenerateResponseBuilder::accounts_blocking)
    /// - [`ixbrl_mandatory`](PostV1DeclarationsIeCt1GenerateResponseBuilder::ixbrl_mandatory)
    /// - [`criteria`](PostV1DeclarationsIeCt1GenerateResponseBuilder::criteria)
    /// - [`fields`](PostV1DeclarationsIeCt1GenerateResponseBuilder::fields)
    /// - [`warnings`](PostV1DeclarationsIeCt1GenerateResponseBuilder::warnings)
    /// - [`notes`](PostV1DeclarationsIeCt1GenerateResponseBuilder::notes)
    /// - [`source`](PostV1DeclarationsIeCt1GenerateResponseBuilder::source)
    pub fn build(self) -> Result<PostV1DeclarationsIeCt1GenerateResponse, BuildError> {
        Ok(PostV1DeclarationsIeCt1GenerateResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            period_start: self
                .period_start
                .ok_or_else(|| BuildError::missing_field("period_start"))?,
            period_end: self
                .period_end
                .ok_or_else(|| BuildError::missing_field("period_end"))?,
            tax_reg_number: self
                .tax_reg_number
                .ok_or_else(|| BuildError::missing_field("tax_reg_number"))?,
            ct1: self.ct1.ok_or_else(|| BuildError::missing_field("ct1"))?,
            accounts: self.accounts,
            accounts_blocking: self
                .accounts_blocking
                .ok_or_else(|| BuildError::missing_field("accounts_blocking"))?,
            ixbrl_mandatory: self
                .ixbrl_mandatory
                .ok_or_else(|| BuildError::missing_field("ixbrl_mandatory"))?,
            criteria: self
                .criteria
                .ok_or_else(|| BuildError::missing_field("criteria"))?,
            fields: self
                .fields
                .ok_or_else(|| BuildError::missing_field("fields"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            notes: self
                .notes
                .ok_or_else(|| BuildError::missing_field("notes"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}

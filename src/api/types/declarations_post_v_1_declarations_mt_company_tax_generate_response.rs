pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsMtCompanyTaxGenerateResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "yearOfAssessment")]
    #[serde(default)]
    pub year_of_assessment: i64,
    #[serde(rename = "periodStart")]
    #[serde(default)]
    pub period_start: String,
    #[serde(rename = "periodEnd")]
    #[serde(default)]
    pub period_end: String,
    #[serde(rename = "incomeTaxNumber")]
    #[serde(default)]
    pub income_tax_number: String,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(default)]
    pub xml: String,
    #[serde(default)]
    pub fields: Vec<PostV1DeclarationsMtCompanyTaxGenerateResponseFieldsItem>,
    #[serde(rename = "taxAccounts")]
    #[serde(default)]
    pub tax_accounts: Vec<PostV1DeclarationsMtCompanyTaxGenerateResponseTaxAccountsItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl PostV1DeclarationsMtCompanyTaxGenerateResponse {
    pub fn builder() -> PostV1DeclarationsMtCompanyTaxGenerateResponseBuilder {
        <PostV1DeclarationsMtCompanyTaxGenerateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsMtCompanyTaxGenerateResponseBuilder {
    year: Option<i64>,
    year_of_assessment: Option<i64>,
    period_start: Option<String>,
    period_end: Option<String>,
    income_tax_number: Option<String>,
    file_name: Option<String>,
    xml: Option<String>,
    fields: Option<Vec<PostV1DeclarationsMtCompanyTaxGenerateResponseFieldsItem>>,
    tax_accounts: Option<Vec<PostV1DeclarationsMtCompanyTaxGenerateResponseTaxAccountsItem>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
}

impl PostV1DeclarationsMtCompanyTaxGenerateResponseBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn year_of_assessment(mut self, value: i64) -> Self {
        self.year_of_assessment = Some(value);
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

    pub fn income_tax_number(mut self, value: impl Into<String>) -> Self {
        self.income_tax_number = Some(value.into());
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn xml(mut self, value: impl Into<String>) -> Self {
        self.xml = Some(value.into());
        self
    }

    pub fn fields(
        mut self,
        value: Vec<PostV1DeclarationsMtCompanyTaxGenerateResponseFieldsItem>,
    ) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn tax_accounts(
        mut self,
        value: Vec<PostV1DeclarationsMtCompanyTaxGenerateResponseTaxAccountsItem>,
    ) -> Self {
        self.tax_accounts = Some(value);
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsMtCompanyTaxGenerateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsMtCompanyTaxGenerateResponseBuilder::year)
    /// - [`year_of_assessment`](PostV1DeclarationsMtCompanyTaxGenerateResponseBuilder::year_of_assessment)
    /// - [`period_start`](PostV1DeclarationsMtCompanyTaxGenerateResponseBuilder::period_start)
    /// - [`period_end`](PostV1DeclarationsMtCompanyTaxGenerateResponseBuilder::period_end)
    /// - [`income_tax_number`](PostV1DeclarationsMtCompanyTaxGenerateResponseBuilder::income_tax_number)
    /// - [`file_name`](PostV1DeclarationsMtCompanyTaxGenerateResponseBuilder::file_name)
    /// - [`xml`](PostV1DeclarationsMtCompanyTaxGenerateResponseBuilder::xml)
    /// - [`fields`](PostV1DeclarationsMtCompanyTaxGenerateResponseBuilder::fields)
    /// - [`tax_accounts`](PostV1DeclarationsMtCompanyTaxGenerateResponseBuilder::tax_accounts)
    /// - [`warnings`](PostV1DeclarationsMtCompanyTaxGenerateResponseBuilder::warnings)
    /// - [`notes`](PostV1DeclarationsMtCompanyTaxGenerateResponseBuilder::notes)
    /// - [`source`](PostV1DeclarationsMtCompanyTaxGenerateResponseBuilder::source)
    pub fn build(self) -> Result<PostV1DeclarationsMtCompanyTaxGenerateResponse, BuildError> {
        Ok(PostV1DeclarationsMtCompanyTaxGenerateResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            year_of_assessment: self
                .year_of_assessment
                .ok_or_else(|| BuildError::missing_field("year_of_assessment"))?,
            period_start: self
                .period_start
                .ok_or_else(|| BuildError::missing_field("period_start"))?,
            period_end: self
                .period_end
                .ok_or_else(|| BuildError::missing_field("period_end"))?,
            income_tax_number: self
                .income_tax_number
                .ok_or_else(|| BuildError::missing_field("income_tax_number"))?,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            xml: self.xml.ok_or_else(|| BuildError::missing_field("xml"))?,
            fields: self
                .fields
                .ok_or_else(|| BuildError::missing_field("fields"))?,
            tax_accounts: self
                .tax_accounts
                .ok_or_else(|| BuildError::missing_field("tax_accounts"))?,
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

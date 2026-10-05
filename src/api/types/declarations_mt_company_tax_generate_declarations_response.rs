pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MtCompanyTaxGenerateDeclarationsResponse {
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
    pub fields: Vec<MtCompanyTaxGenerateDeclarationsResponseFieldsItem>,
    #[serde(rename = "taxAccounts")]
    #[serde(default)]
    pub tax_accounts: Vec<MtCompanyTaxGenerateDeclarationsResponseTaxAccountsItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl MtCompanyTaxGenerateDeclarationsResponse {
    pub fn builder() -> MtCompanyTaxGenerateDeclarationsResponseBuilder {
        <MtCompanyTaxGenerateDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MtCompanyTaxGenerateDeclarationsResponseBuilder {
    year: Option<i64>,
    year_of_assessment: Option<i64>,
    period_start: Option<String>,
    period_end: Option<String>,
    income_tax_number: Option<String>,
    file_name: Option<String>,
    xml: Option<String>,
    fields: Option<Vec<MtCompanyTaxGenerateDeclarationsResponseFieldsItem>>,
    tax_accounts: Option<Vec<MtCompanyTaxGenerateDeclarationsResponseTaxAccountsItem>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
}

impl MtCompanyTaxGenerateDeclarationsResponseBuilder {
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
        value: Vec<MtCompanyTaxGenerateDeclarationsResponseFieldsItem>,
    ) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn tax_accounts(
        mut self,
        value: Vec<MtCompanyTaxGenerateDeclarationsResponseTaxAccountsItem>,
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

    /// Consumes the builder and constructs a [`MtCompanyTaxGenerateDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](MtCompanyTaxGenerateDeclarationsResponseBuilder::year)
    /// - [`year_of_assessment`](MtCompanyTaxGenerateDeclarationsResponseBuilder::year_of_assessment)
    /// - [`period_start`](MtCompanyTaxGenerateDeclarationsResponseBuilder::period_start)
    /// - [`period_end`](MtCompanyTaxGenerateDeclarationsResponseBuilder::period_end)
    /// - [`income_tax_number`](MtCompanyTaxGenerateDeclarationsResponseBuilder::income_tax_number)
    /// - [`file_name`](MtCompanyTaxGenerateDeclarationsResponseBuilder::file_name)
    /// - [`xml`](MtCompanyTaxGenerateDeclarationsResponseBuilder::xml)
    /// - [`fields`](MtCompanyTaxGenerateDeclarationsResponseBuilder::fields)
    /// - [`tax_accounts`](MtCompanyTaxGenerateDeclarationsResponseBuilder::tax_accounts)
    /// - [`warnings`](MtCompanyTaxGenerateDeclarationsResponseBuilder::warnings)
    /// - [`notes`](MtCompanyTaxGenerateDeclarationsResponseBuilder::notes)
    /// - [`source`](MtCompanyTaxGenerateDeclarationsResponseBuilder::source)
    pub fn build(self) -> Result<MtCompanyTaxGenerateDeclarationsResponse, BuildError> {
        Ok(MtCompanyTaxGenerateDeclarationsResponse {
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

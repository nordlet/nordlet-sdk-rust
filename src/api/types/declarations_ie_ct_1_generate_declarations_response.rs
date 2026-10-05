pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct IeCt1GenerateDeclarationsResponse {
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
    pub ct1: IeCt1GenerateDeclarationsResponseCt1,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<IeCt1GenerateDeclarationsResponseAccounts>,
    #[serde(rename = "accountsBlocking")]
    #[serde(default)]
    pub accounts_blocking: Vec<String>,
    #[serde(rename = "ixbrlMandatory")]
    #[serde(default)]
    pub ixbrl_mandatory: bool,
    #[serde(default)]
    pub criteria: IeCt1GenerateDeclarationsResponseCriteria,
    #[serde(default)]
    pub fields: Vec<IeCt1GenerateDeclarationsResponseFieldsItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl IeCt1GenerateDeclarationsResponse {
    pub fn builder() -> IeCt1GenerateDeclarationsResponseBuilder {
        <IeCt1GenerateDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IeCt1GenerateDeclarationsResponseBuilder {
    year: Option<i64>,
    period_start: Option<String>,
    period_end: Option<String>,
    tax_reg_number: Option<String>,
    ct1: Option<IeCt1GenerateDeclarationsResponseCt1>,
    accounts: Option<IeCt1GenerateDeclarationsResponseAccounts>,
    accounts_blocking: Option<Vec<String>>,
    ixbrl_mandatory: Option<bool>,
    criteria: Option<IeCt1GenerateDeclarationsResponseCriteria>,
    fields: Option<Vec<IeCt1GenerateDeclarationsResponseFieldsItem>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
}

impl IeCt1GenerateDeclarationsResponseBuilder {
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

    pub fn ct1(mut self, value: IeCt1GenerateDeclarationsResponseCt1) -> Self {
        self.ct1 = Some(value);
        self
    }

    pub fn accounts(mut self, value: IeCt1GenerateDeclarationsResponseAccounts) -> Self {
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

    pub fn criteria(mut self, value: IeCt1GenerateDeclarationsResponseCriteria) -> Self {
        self.criteria = Some(value);
        self
    }

    pub fn fields(mut self, value: Vec<IeCt1GenerateDeclarationsResponseFieldsItem>) -> Self {
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

    /// Consumes the builder and constructs a [`IeCt1GenerateDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](IeCt1GenerateDeclarationsResponseBuilder::year)
    /// - [`period_start`](IeCt1GenerateDeclarationsResponseBuilder::period_start)
    /// - [`period_end`](IeCt1GenerateDeclarationsResponseBuilder::period_end)
    /// - [`tax_reg_number`](IeCt1GenerateDeclarationsResponseBuilder::tax_reg_number)
    /// - [`ct1`](IeCt1GenerateDeclarationsResponseBuilder::ct1)
    /// - [`accounts_blocking`](IeCt1GenerateDeclarationsResponseBuilder::accounts_blocking)
    /// - [`ixbrl_mandatory`](IeCt1GenerateDeclarationsResponseBuilder::ixbrl_mandatory)
    /// - [`criteria`](IeCt1GenerateDeclarationsResponseBuilder::criteria)
    /// - [`fields`](IeCt1GenerateDeclarationsResponseBuilder::fields)
    /// - [`warnings`](IeCt1GenerateDeclarationsResponseBuilder::warnings)
    /// - [`notes`](IeCt1GenerateDeclarationsResponseBuilder::notes)
    /// - [`source`](IeCt1GenerateDeclarationsResponseBuilder::source)
    pub fn build(self) -> Result<IeCt1GenerateDeclarationsResponse, BuildError> {
        Ok(IeCt1GenerateDeclarationsResponse {
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

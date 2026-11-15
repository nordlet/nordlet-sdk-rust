pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompaniesContextAccountResponse {
    #[serde(default)]
    pub periods: CompaniesContextAccountResponsePeriods,
    #[serde(rename = "postingRules")]
    #[serde(default)]
    pub posting_rules: HashMap<String, String>,
    #[serde(rename = "bankAccounts")]
    #[serde(default)]
    pub bank_accounts: Vec<CompaniesContextAccountResponseBankAccountsItem>,
    #[serde(rename = "vatTypesUsed")]
    #[serde(default)]
    pub vat_types_used: Vec<CompaniesContextAccountResponseVatTypesUsedItem>,
    #[serde(default)]
    pub filings: Vec<CompaniesContextAccountResponseFilingsItem>,
    #[serde(default)]
    pub payroll: CompaniesContextAccountResponsePayroll,
    pub profile: CompaniesContextAccountResponseProfile,
}

impl CompaniesContextAccountResponse {
    pub fn builder() -> CompaniesContextAccountResponseBuilder {
        <CompaniesContextAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompaniesContextAccountResponseBuilder {
    periods: Option<CompaniesContextAccountResponsePeriods>,
    posting_rules: Option<HashMap<String, String>>,
    bank_accounts: Option<Vec<CompaniesContextAccountResponseBankAccountsItem>>,
    vat_types_used: Option<Vec<CompaniesContextAccountResponseVatTypesUsedItem>>,
    filings: Option<Vec<CompaniesContextAccountResponseFilingsItem>>,
    payroll: Option<CompaniesContextAccountResponsePayroll>,
    profile: Option<CompaniesContextAccountResponseProfile>,
}

impl CompaniesContextAccountResponseBuilder {
    pub fn periods(mut self, value: CompaniesContextAccountResponsePeriods) -> Self {
        self.periods = Some(value);
        self
    }

    pub fn posting_rules(mut self, value: HashMap<String, String>) -> Self {
        self.posting_rules = Some(value);
        self
    }

    pub fn bank_accounts(
        mut self,
        value: Vec<CompaniesContextAccountResponseBankAccountsItem>,
    ) -> Self {
        self.bank_accounts = Some(value);
        self
    }

    pub fn vat_types_used(
        mut self,
        value: Vec<CompaniesContextAccountResponseVatTypesUsedItem>,
    ) -> Self {
        self.vat_types_used = Some(value);
        self
    }

    pub fn filings(mut self, value: Vec<CompaniesContextAccountResponseFilingsItem>) -> Self {
        self.filings = Some(value);
        self
    }

    pub fn payroll(mut self, value: CompaniesContextAccountResponsePayroll) -> Self {
        self.payroll = Some(value);
        self
    }

    pub fn profile(mut self, value: CompaniesContextAccountResponseProfile) -> Self {
        self.profile = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompaniesContextAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`periods`](CompaniesContextAccountResponseBuilder::periods)
    /// - [`posting_rules`](CompaniesContextAccountResponseBuilder::posting_rules)
    /// - [`bank_accounts`](CompaniesContextAccountResponseBuilder::bank_accounts)
    /// - [`vat_types_used`](CompaniesContextAccountResponseBuilder::vat_types_used)
    /// - [`filings`](CompaniesContextAccountResponseBuilder::filings)
    /// - [`payroll`](CompaniesContextAccountResponseBuilder::payroll)
    /// - [`profile`](CompaniesContextAccountResponseBuilder::profile)
    pub fn build(self) -> Result<CompaniesContextAccountResponse, BuildError> {
        Ok(CompaniesContextAccountResponse {
            periods: self
                .periods
                .ok_or_else(|| BuildError::missing_field("periods"))?,
            posting_rules: self
                .posting_rules
                .ok_or_else(|| BuildError::missing_field("posting_rules"))?,
            bank_accounts: self
                .bank_accounts
                .ok_or_else(|| BuildError::missing_field("bank_accounts"))?,
            vat_types_used: self
                .vat_types_used
                .ok_or_else(|| BuildError::missing_field("vat_types_used"))?,
            filings: self
                .filings
                .ok_or_else(|| BuildError::missing_field("filings"))?,
            payroll: self
                .payroll
                .ok_or_else(|| BuildError::missing_field("payroll"))?,
            profile: self
                .profile
                .ok_or_else(|| BuildError::missing_field("profile"))?,
        })
    }
}

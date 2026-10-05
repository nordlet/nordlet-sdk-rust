pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BooksValidateMigrationResponse {
    #[serde(rename = "dryRun")]
    #[serde(default)]
    pub dry_run: bool,
    #[serde(rename = "cutoverDate")]
    #[serde(default)]
    pub cutover_date: NaiveDate,
    #[serde(default)]
    pub accounts: BooksValidateMigrationResponseAccounts,
    #[serde(default)]
    pub partners: BooksValidateMigrationResponsePartners,
    #[serde(default)]
    pub items: BooksValidateMigrationResponseItems,
    #[serde(rename = "assetGroups")]
    #[serde(default)]
    pub asset_groups: BooksValidateMigrationResponseAssetGroups,
    #[serde(rename = "openingBalances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opening_balances: Option<BooksValidateMigrationResponseOpeningBalances>,
    #[serde(default)]
    pub journal: BooksValidateMigrationResponseJournal,
    #[serde(rename = "openReceivables")]
    #[serde(default)]
    pub open_receivables: BooksValidateMigrationResponseOpenReceivables,
    #[serde(rename = "openPayables")]
    #[serde(default)]
    pub open_payables: BooksValidateMigrationResponseOpenPayables,
    #[serde(rename = "fixedAssets")]
    #[serde(default)]
    pub fixed_assets: BooksValidateMigrationResponseFixedAssets,
    #[serde(default)]
    pub stock: BooksValidateMigrationResponseStock,
    #[serde(rename = "numberSeries")]
    #[serde(default)]
    pub number_series: Vec<BooksValidateMigrationResponseNumberSeriesItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl BooksValidateMigrationResponse {
    pub fn builder() -> BooksValidateMigrationResponseBuilder {
        <BooksValidateMigrationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BooksValidateMigrationResponseBuilder {
    dry_run: Option<bool>,
    cutover_date: Option<NaiveDate>,
    accounts: Option<BooksValidateMigrationResponseAccounts>,
    partners: Option<BooksValidateMigrationResponsePartners>,
    items: Option<BooksValidateMigrationResponseItems>,
    asset_groups: Option<BooksValidateMigrationResponseAssetGroups>,
    opening_balances: Option<BooksValidateMigrationResponseOpeningBalances>,
    journal: Option<BooksValidateMigrationResponseJournal>,
    open_receivables: Option<BooksValidateMigrationResponseOpenReceivables>,
    open_payables: Option<BooksValidateMigrationResponseOpenPayables>,
    fixed_assets: Option<BooksValidateMigrationResponseFixedAssets>,
    stock: Option<BooksValidateMigrationResponseStock>,
    number_series: Option<Vec<BooksValidateMigrationResponseNumberSeriesItem>>,
    warnings: Option<Vec<String>>,
}

impl BooksValidateMigrationResponseBuilder {
    pub fn dry_run(mut self, value: bool) -> Self {
        self.dry_run = Some(value);
        self
    }

    pub fn cutover_date(mut self, value: NaiveDate) -> Self {
        self.cutover_date = Some(value);
        self
    }

    pub fn accounts(mut self, value: BooksValidateMigrationResponseAccounts) -> Self {
        self.accounts = Some(value);
        self
    }

    pub fn partners(mut self, value: BooksValidateMigrationResponsePartners) -> Self {
        self.partners = Some(value);
        self
    }

    pub fn items(mut self, value: BooksValidateMigrationResponseItems) -> Self {
        self.items = Some(value);
        self
    }

    pub fn asset_groups(mut self, value: BooksValidateMigrationResponseAssetGroups) -> Self {
        self.asset_groups = Some(value);
        self
    }

    pub fn opening_balances(
        mut self,
        value: BooksValidateMigrationResponseOpeningBalances,
    ) -> Self {
        self.opening_balances = Some(value);
        self
    }

    pub fn journal(mut self, value: BooksValidateMigrationResponseJournal) -> Self {
        self.journal = Some(value);
        self
    }

    pub fn open_receivables(
        mut self,
        value: BooksValidateMigrationResponseOpenReceivables,
    ) -> Self {
        self.open_receivables = Some(value);
        self
    }

    pub fn open_payables(mut self, value: BooksValidateMigrationResponseOpenPayables) -> Self {
        self.open_payables = Some(value);
        self
    }

    pub fn fixed_assets(mut self, value: BooksValidateMigrationResponseFixedAssets) -> Self {
        self.fixed_assets = Some(value);
        self
    }

    pub fn stock(mut self, value: BooksValidateMigrationResponseStock) -> Self {
        self.stock = Some(value);
        self
    }

    pub fn number_series(
        mut self,
        value: Vec<BooksValidateMigrationResponseNumberSeriesItem>,
    ) -> Self {
        self.number_series = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BooksValidateMigrationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dry_run`](BooksValidateMigrationResponseBuilder::dry_run)
    /// - [`cutover_date`](BooksValidateMigrationResponseBuilder::cutover_date)
    /// - [`accounts`](BooksValidateMigrationResponseBuilder::accounts)
    /// - [`partners`](BooksValidateMigrationResponseBuilder::partners)
    /// - [`items`](BooksValidateMigrationResponseBuilder::items)
    /// - [`asset_groups`](BooksValidateMigrationResponseBuilder::asset_groups)
    /// - [`journal`](BooksValidateMigrationResponseBuilder::journal)
    /// - [`open_receivables`](BooksValidateMigrationResponseBuilder::open_receivables)
    /// - [`open_payables`](BooksValidateMigrationResponseBuilder::open_payables)
    /// - [`fixed_assets`](BooksValidateMigrationResponseBuilder::fixed_assets)
    /// - [`stock`](BooksValidateMigrationResponseBuilder::stock)
    /// - [`number_series`](BooksValidateMigrationResponseBuilder::number_series)
    /// - [`warnings`](BooksValidateMigrationResponseBuilder::warnings)
    pub fn build(self) -> Result<BooksValidateMigrationResponse, BuildError> {
        Ok(BooksValidateMigrationResponse {
            dry_run: self
                .dry_run
                .ok_or_else(|| BuildError::missing_field("dry_run"))?,
            cutover_date: self
                .cutover_date
                .ok_or_else(|| BuildError::missing_field("cutover_date"))?,
            accounts: self
                .accounts
                .ok_or_else(|| BuildError::missing_field("accounts"))?,
            partners: self
                .partners
                .ok_or_else(|| BuildError::missing_field("partners"))?,
            items: self
                .items
                .ok_or_else(|| BuildError::missing_field("items"))?,
            asset_groups: self
                .asset_groups
                .ok_or_else(|| BuildError::missing_field("asset_groups"))?,
            opening_balances: self.opening_balances,
            journal: self
                .journal
                .ok_or_else(|| BuildError::missing_field("journal"))?,
            open_receivables: self
                .open_receivables
                .ok_or_else(|| BuildError::missing_field("open_receivables"))?,
            open_payables: self
                .open_payables
                .ok_or_else(|| BuildError::missing_field("open_payables"))?,
            fixed_assets: self
                .fixed_assets
                .ok_or_else(|| BuildError::missing_field("fixed_assets"))?,
            stock: self
                .stock
                .ok_or_else(|| BuildError::missing_field("stock"))?,
            number_series: self
                .number_series
                .ok_or_else(|| BuildError::missing_field("number_series"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
        })
    }
}

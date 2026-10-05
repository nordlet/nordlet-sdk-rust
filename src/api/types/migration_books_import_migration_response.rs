pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BooksImportMigrationResponse {
    #[serde(rename = "dryRun")]
    #[serde(default)]
    pub dry_run: bool,
    #[serde(rename = "cutoverDate")]
    #[serde(default)]
    pub cutover_date: NaiveDate,
    #[serde(default)]
    pub accounts: BooksImportMigrationResponseAccounts,
    #[serde(default)]
    pub partners: BooksImportMigrationResponsePartners,
    #[serde(default)]
    pub items: BooksImportMigrationResponseItems,
    #[serde(rename = "assetGroups")]
    #[serde(default)]
    pub asset_groups: BooksImportMigrationResponseAssetGroups,
    #[serde(rename = "openingBalances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opening_balances: Option<BooksImportMigrationResponseOpeningBalances>,
    #[serde(default)]
    pub journal: BooksImportMigrationResponseJournal,
    #[serde(rename = "openReceivables")]
    #[serde(default)]
    pub open_receivables: BooksImportMigrationResponseOpenReceivables,
    #[serde(rename = "openPayables")]
    #[serde(default)]
    pub open_payables: BooksImportMigrationResponseOpenPayables,
    #[serde(rename = "fixedAssets")]
    #[serde(default)]
    pub fixed_assets: BooksImportMigrationResponseFixedAssets,
    #[serde(default)]
    pub stock: BooksImportMigrationResponseStock,
    #[serde(rename = "numberSeries")]
    #[serde(default)]
    pub number_series: Vec<BooksImportMigrationResponseNumberSeriesItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl BooksImportMigrationResponse {
    pub fn builder() -> BooksImportMigrationResponseBuilder {
        <BooksImportMigrationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BooksImportMigrationResponseBuilder {
    dry_run: Option<bool>,
    cutover_date: Option<NaiveDate>,
    accounts: Option<BooksImportMigrationResponseAccounts>,
    partners: Option<BooksImportMigrationResponsePartners>,
    items: Option<BooksImportMigrationResponseItems>,
    asset_groups: Option<BooksImportMigrationResponseAssetGroups>,
    opening_balances: Option<BooksImportMigrationResponseOpeningBalances>,
    journal: Option<BooksImportMigrationResponseJournal>,
    open_receivables: Option<BooksImportMigrationResponseOpenReceivables>,
    open_payables: Option<BooksImportMigrationResponseOpenPayables>,
    fixed_assets: Option<BooksImportMigrationResponseFixedAssets>,
    stock: Option<BooksImportMigrationResponseStock>,
    number_series: Option<Vec<BooksImportMigrationResponseNumberSeriesItem>>,
    warnings: Option<Vec<String>>,
}

impl BooksImportMigrationResponseBuilder {
    pub fn dry_run(mut self, value: bool) -> Self {
        self.dry_run = Some(value);
        self
    }

    pub fn cutover_date(mut self, value: NaiveDate) -> Self {
        self.cutover_date = Some(value);
        self
    }

    pub fn accounts(mut self, value: BooksImportMigrationResponseAccounts) -> Self {
        self.accounts = Some(value);
        self
    }

    pub fn partners(mut self, value: BooksImportMigrationResponsePartners) -> Self {
        self.partners = Some(value);
        self
    }

    pub fn items(mut self, value: BooksImportMigrationResponseItems) -> Self {
        self.items = Some(value);
        self
    }

    pub fn asset_groups(mut self, value: BooksImportMigrationResponseAssetGroups) -> Self {
        self.asset_groups = Some(value);
        self
    }

    pub fn opening_balances(mut self, value: BooksImportMigrationResponseOpeningBalances) -> Self {
        self.opening_balances = Some(value);
        self
    }

    pub fn journal(mut self, value: BooksImportMigrationResponseJournal) -> Self {
        self.journal = Some(value);
        self
    }

    pub fn open_receivables(mut self, value: BooksImportMigrationResponseOpenReceivables) -> Self {
        self.open_receivables = Some(value);
        self
    }

    pub fn open_payables(mut self, value: BooksImportMigrationResponseOpenPayables) -> Self {
        self.open_payables = Some(value);
        self
    }

    pub fn fixed_assets(mut self, value: BooksImportMigrationResponseFixedAssets) -> Self {
        self.fixed_assets = Some(value);
        self
    }

    pub fn stock(mut self, value: BooksImportMigrationResponseStock) -> Self {
        self.stock = Some(value);
        self
    }

    pub fn number_series(
        mut self,
        value: Vec<BooksImportMigrationResponseNumberSeriesItem>,
    ) -> Self {
        self.number_series = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BooksImportMigrationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dry_run`](BooksImportMigrationResponseBuilder::dry_run)
    /// - [`cutover_date`](BooksImportMigrationResponseBuilder::cutover_date)
    /// - [`accounts`](BooksImportMigrationResponseBuilder::accounts)
    /// - [`partners`](BooksImportMigrationResponseBuilder::partners)
    /// - [`items`](BooksImportMigrationResponseBuilder::items)
    /// - [`asset_groups`](BooksImportMigrationResponseBuilder::asset_groups)
    /// - [`journal`](BooksImportMigrationResponseBuilder::journal)
    /// - [`open_receivables`](BooksImportMigrationResponseBuilder::open_receivables)
    /// - [`open_payables`](BooksImportMigrationResponseBuilder::open_payables)
    /// - [`fixed_assets`](BooksImportMigrationResponseBuilder::fixed_assets)
    /// - [`stock`](BooksImportMigrationResponseBuilder::stock)
    /// - [`number_series`](BooksImportMigrationResponseBuilder::number_series)
    /// - [`warnings`](BooksImportMigrationResponseBuilder::warnings)
    pub fn build(self) -> Result<BooksImportMigrationResponse, BuildError> {
        Ok(BooksImportMigrationResponse {
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

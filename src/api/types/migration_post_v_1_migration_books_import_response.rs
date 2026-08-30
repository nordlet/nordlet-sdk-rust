pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1MigrationBooksImportResponse {
    #[serde(rename = "dryRun")]
    #[serde(default)]
    pub dry_run: bool,
    #[serde(rename = "cutoverDate")]
    #[serde(default)]
    pub cutover_date: String,
    #[serde(default)]
    pub accounts: PostV1MigrationBooksImportResponseAccounts,
    #[serde(default)]
    pub partners: PostV1MigrationBooksImportResponsePartners,
    #[serde(default)]
    pub items: PostV1MigrationBooksImportResponseItems,
    #[serde(rename = "assetGroups")]
    #[serde(default)]
    pub asset_groups: PostV1MigrationBooksImportResponseAssetGroups,
    #[serde(rename = "openingBalances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opening_balances: Option<PostV1MigrationBooksImportResponseOpeningBalances>,
    #[serde(default)]
    pub journal: PostV1MigrationBooksImportResponseJournal,
    #[serde(rename = "openReceivables")]
    #[serde(default)]
    pub open_receivables: PostV1MigrationBooksImportResponseOpenReceivables,
    #[serde(rename = "openPayables")]
    #[serde(default)]
    pub open_payables: PostV1MigrationBooksImportResponseOpenPayables,
    #[serde(rename = "fixedAssets")]
    #[serde(default)]
    pub fixed_assets: PostV1MigrationBooksImportResponseFixedAssets,
    #[serde(default)]
    pub stock: PostV1MigrationBooksImportResponseStock,
    #[serde(rename = "numberSeries")]
    #[serde(default)]
    pub number_series: Vec<PostV1MigrationBooksImportResponseNumberSeriesItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl PostV1MigrationBooksImportResponse {
    pub fn builder() -> PostV1MigrationBooksImportResponseBuilder {
        <PostV1MigrationBooksImportResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1MigrationBooksImportResponseBuilder {
    dry_run: Option<bool>,
    cutover_date: Option<String>,
    accounts: Option<PostV1MigrationBooksImportResponseAccounts>,
    partners: Option<PostV1MigrationBooksImportResponsePartners>,
    items: Option<PostV1MigrationBooksImportResponseItems>,
    asset_groups: Option<PostV1MigrationBooksImportResponseAssetGroups>,
    opening_balances: Option<PostV1MigrationBooksImportResponseOpeningBalances>,
    journal: Option<PostV1MigrationBooksImportResponseJournal>,
    open_receivables: Option<PostV1MigrationBooksImportResponseOpenReceivables>,
    open_payables: Option<PostV1MigrationBooksImportResponseOpenPayables>,
    fixed_assets: Option<PostV1MigrationBooksImportResponseFixedAssets>,
    stock: Option<PostV1MigrationBooksImportResponseStock>,
    number_series: Option<Vec<PostV1MigrationBooksImportResponseNumberSeriesItem>>,
    warnings: Option<Vec<String>>,
}

impl PostV1MigrationBooksImportResponseBuilder {
    pub fn dry_run(mut self, value: bool) -> Self {
        self.dry_run = Some(value);
        self
    }

    pub fn cutover_date(mut self, value: impl Into<String>) -> Self {
        self.cutover_date = Some(value.into());
        self
    }

    pub fn accounts(mut self, value: PostV1MigrationBooksImportResponseAccounts) -> Self {
        self.accounts = Some(value);
        self
    }

    pub fn partners(mut self, value: PostV1MigrationBooksImportResponsePartners) -> Self {
        self.partners = Some(value);
        self
    }

    pub fn items(mut self, value: PostV1MigrationBooksImportResponseItems) -> Self {
        self.items = Some(value);
        self
    }

    pub fn asset_groups(mut self, value: PostV1MigrationBooksImportResponseAssetGroups) -> Self {
        self.asset_groups = Some(value);
        self
    }

    pub fn opening_balances(
        mut self,
        value: PostV1MigrationBooksImportResponseOpeningBalances,
    ) -> Self {
        self.opening_balances = Some(value);
        self
    }

    pub fn journal(mut self, value: PostV1MigrationBooksImportResponseJournal) -> Self {
        self.journal = Some(value);
        self
    }

    pub fn open_receivables(
        mut self,
        value: PostV1MigrationBooksImportResponseOpenReceivables,
    ) -> Self {
        self.open_receivables = Some(value);
        self
    }

    pub fn open_payables(mut self, value: PostV1MigrationBooksImportResponseOpenPayables) -> Self {
        self.open_payables = Some(value);
        self
    }

    pub fn fixed_assets(mut self, value: PostV1MigrationBooksImportResponseFixedAssets) -> Self {
        self.fixed_assets = Some(value);
        self
    }

    pub fn stock(mut self, value: PostV1MigrationBooksImportResponseStock) -> Self {
        self.stock = Some(value);
        self
    }

    pub fn number_series(
        mut self,
        value: Vec<PostV1MigrationBooksImportResponseNumberSeriesItem>,
    ) -> Self {
        self.number_series = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1MigrationBooksImportResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dry_run`](PostV1MigrationBooksImportResponseBuilder::dry_run)
    /// - [`cutover_date`](PostV1MigrationBooksImportResponseBuilder::cutover_date)
    /// - [`accounts`](PostV1MigrationBooksImportResponseBuilder::accounts)
    /// - [`partners`](PostV1MigrationBooksImportResponseBuilder::partners)
    /// - [`items`](PostV1MigrationBooksImportResponseBuilder::items)
    /// - [`asset_groups`](PostV1MigrationBooksImportResponseBuilder::asset_groups)
    /// - [`journal`](PostV1MigrationBooksImportResponseBuilder::journal)
    /// - [`open_receivables`](PostV1MigrationBooksImportResponseBuilder::open_receivables)
    /// - [`open_payables`](PostV1MigrationBooksImportResponseBuilder::open_payables)
    /// - [`fixed_assets`](PostV1MigrationBooksImportResponseBuilder::fixed_assets)
    /// - [`stock`](PostV1MigrationBooksImportResponseBuilder::stock)
    /// - [`number_series`](PostV1MigrationBooksImportResponseBuilder::number_series)
    /// - [`warnings`](PostV1MigrationBooksImportResponseBuilder::warnings)
    pub fn build(self) -> Result<PostV1MigrationBooksImportResponse, BuildError> {
        Ok(PostV1MigrationBooksImportResponse {
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

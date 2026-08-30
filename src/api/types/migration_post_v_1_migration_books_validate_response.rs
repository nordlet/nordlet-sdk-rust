pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1MigrationBooksValidateResponse {
    #[serde(rename = "dryRun")]
    #[serde(default)]
    pub dry_run: bool,
    #[serde(rename = "cutoverDate")]
    #[serde(default)]
    pub cutover_date: String,
    #[serde(default)]
    pub accounts: PostV1MigrationBooksValidateResponseAccounts,
    #[serde(default)]
    pub partners: PostV1MigrationBooksValidateResponsePartners,
    #[serde(default)]
    pub items: PostV1MigrationBooksValidateResponseItems,
    #[serde(rename = "assetGroups")]
    #[serde(default)]
    pub asset_groups: PostV1MigrationBooksValidateResponseAssetGroups,
    #[serde(rename = "openingBalances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opening_balances: Option<PostV1MigrationBooksValidateResponseOpeningBalances>,
    #[serde(default)]
    pub journal: PostV1MigrationBooksValidateResponseJournal,
    #[serde(rename = "openReceivables")]
    #[serde(default)]
    pub open_receivables: PostV1MigrationBooksValidateResponseOpenReceivables,
    #[serde(rename = "openPayables")]
    #[serde(default)]
    pub open_payables: PostV1MigrationBooksValidateResponseOpenPayables,
    #[serde(rename = "fixedAssets")]
    #[serde(default)]
    pub fixed_assets: PostV1MigrationBooksValidateResponseFixedAssets,
    #[serde(default)]
    pub stock: PostV1MigrationBooksValidateResponseStock,
    #[serde(rename = "numberSeries")]
    #[serde(default)]
    pub number_series: Vec<PostV1MigrationBooksValidateResponseNumberSeriesItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl PostV1MigrationBooksValidateResponse {
    pub fn builder() -> PostV1MigrationBooksValidateResponseBuilder {
        <PostV1MigrationBooksValidateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1MigrationBooksValidateResponseBuilder {
    dry_run: Option<bool>,
    cutover_date: Option<String>,
    accounts: Option<PostV1MigrationBooksValidateResponseAccounts>,
    partners: Option<PostV1MigrationBooksValidateResponsePartners>,
    items: Option<PostV1MigrationBooksValidateResponseItems>,
    asset_groups: Option<PostV1MigrationBooksValidateResponseAssetGroups>,
    opening_balances: Option<PostV1MigrationBooksValidateResponseOpeningBalances>,
    journal: Option<PostV1MigrationBooksValidateResponseJournal>,
    open_receivables: Option<PostV1MigrationBooksValidateResponseOpenReceivables>,
    open_payables: Option<PostV1MigrationBooksValidateResponseOpenPayables>,
    fixed_assets: Option<PostV1MigrationBooksValidateResponseFixedAssets>,
    stock: Option<PostV1MigrationBooksValidateResponseStock>,
    number_series: Option<Vec<PostV1MigrationBooksValidateResponseNumberSeriesItem>>,
    warnings: Option<Vec<String>>,
}

impl PostV1MigrationBooksValidateResponseBuilder {
    pub fn dry_run(mut self, value: bool) -> Self {
        self.dry_run = Some(value);
        self
    }

    pub fn cutover_date(mut self, value: impl Into<String>) -> Self {
        self.cutover_date = Some(value.into());
        self
    }

    pub fn accounts(mut self, value: PostV1MigrationBooksValidateResponseAccounts) -> Self {
        self.accounts = Some(value);
        self
    }

    pub fn partners(mut self, value: PostV1MigrationBooksValidateResponsePartners) -> Self {
        self.partners = Some(value);
        self
    }

    pub fn items(mut self, value: PostV1MigrationBooksValidateResponseItems) -> Self {
        self.items = Some(value);
        self
    }

    pub fn asset_groups(mut self, value: PostV1MigrationBooksValidateResponseAssetGroups) -> Self {
        self.asset_groups = Some(value);
        self
    }

    pub fn opening_balances(
        mut self,
        value: PostV1MigrationBooksValidateResponseOpeningBalances,
    ) -> Self {
        self.opening_balances = Some(value);
        self
    }

    pub fn journal(mut self, value: PostV1MigrationBooksValidateResponseJournal) -> Self {
        self.journal = Some(value);
        self
    }

    pub fn open_receivables(
        mut self,
        value: PostV1MigrationBooksValidateResponseOpenReceivables,
    ) -> Self {
        self.open_receivables = Some(value);
        self
    }

    pub fn open_payables(
        mut self,
        value: PostV1MigrationBooksValidateResponseOpenPayables,
    ) -> Self {
        self.open_payables = Some(value);
        self
    }

    pub fn fixed_assets(mut self, value: PostV1MigrationBooksValidateResponseFixedAssets) -> Self {
        self.fixed_assets = Some(value);
        self
    }

    pub fn stock(mut self, value: PostV1MigrationBooksValidateResponseStock) -> Self {
        self.stock = Some(value);
        self
    }

    pub fn number_series(
        mut self,
        value: Vec<PostV1MigrationBooksValidateResponseNumberSeriesItem>,
    ) -> Self {
        self.number_series = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1MigrationBooksValidateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dry_run`](PostV1MigrationBooksValidateResponseBuilder::dry_run)
    /// - [`cutover_date`](PostV1MigrationBooksValidateResponseBuilder::cutover_date)
    /// - [`accounts`](PostV1MigrationBooksValidateResponseBuilder::accounts)
    /// - [`partners`](PostV1MigrationBooksValidateResponseBuilder::partners)
    /// - [`items`](PostV1MigrationBooksValidateResponseBuilder::items)
    /// - [`asset_groups`](PostV1MigrationBooksValidateResponseBuilder::asset_groups)
    /// - [`journal`](PostV1MigrationBooksValidateResponseBuilder::journal)
    /// - [`open_receivables`](PostV1MigrationBooksValidateResponseBuilder::open_receivables)
    /// - [`open_payables`](PostV1MigrationBooksValidateResponseBuilder::open_payables)
    /// - [`fixed_assets`](PostV1MigrationBooksValidateResponseBuilder::fixed_assets)
    /// - [`stock`](PostV1MigrationBooksValidateResponseBuilder::stock)
    /// - [`number_series`](PostV1MigrationBooksValidateResponseBuilder::number_series)
    /// - [`warnings`](PostV1MigrationBooksValidateResponseBuilder::warnings)
    pub fn build(self) -> Result<PostV1MigrationBooksValidateResponse, BuildError> {
        Ok(PostV1MigrationBooksValidateResponse {
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

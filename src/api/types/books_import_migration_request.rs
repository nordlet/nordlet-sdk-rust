pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BooksImportMigrationRequest {
    #[serde(rename = "cutoverDate")]
    #[serde(default)]
    pub cutover_date: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<BooksImportMigrationRequestAccountsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partners: Option<Vec<BooksImportMigrationRequestPartnersItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<BooksImportMigrationRequestItemsItem>>,
    #[serde(rename = "openingBalances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opening_balances: Option<BooksImportMigrationRequestOpeningBalances>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal: Option<Vec<BooksImportMigrationRequestJournalItem>>,
    #[serde(rename = "openReceivables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_receivables: Option<Vec<BooksImportMigrationRequestOpenReceivablesItem>>,
    #[serde(rename = "openPayables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_payables: Option<Vec<BooksImportMigrationRequestOpenPayablesItem>>,
    #[serde(rename = "assetGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_groups: Option<Vec<BooksImportMigrationRequestAssetGroupsItem>>,
    #[serde(rename = "fixedAssets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_assets: Option<Vec<BooksImportMigrationRequestFixedAssetsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock: Option<Vec<BooksImportMigrationRequestStockItem>>,
}

impl BooksImportMigrationRequest {
    pub fn builder() -> BooksImportMigrationRequestBuilder {
        <BooksImportMigrationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BooksImportMigrationRequestBuilder {
    cutover_date: Option<NaiveDate>,
    source: Option<String>,
    accounts: Option<Vec<BooksImportMigrationRequestAccountsItem>>,
    partners: Option<Vec<BooksImportMigrationRequestPartnersItem>>,
    items: Option<Vec<BooksImportMigrationRequestItemsItem>>,
    opening_balances: Option<BooksImportMigrationRequestOpeningBalances>,
    journal: Option<Vec<BooksImportMigrationRequestJournalItem>>,
    open_receivables: Option<Vec<BooksImportMigrationRequestOpenReceivablesItem>>,
    open_payables: Option<Vec<BooksImportMigrationRequestOpenPayablesItem>>,
    asset_groups: Option<Vec<BooksImportMigrationRequestAssetGroupsItem>>,
    fixed_assets: Option<Vec<BooksImportMigrationRequestFixedAssetsItem>>,
    stock: Option<Vec<BooksImportMigrationRequestStockItem>>,
}

impl BooksImportMigrationRequestBuilder {
    pub fn cutover_date(mut self, value: NaiveDate) -> Self {
        self.cutover_date = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn accounts(mut self, value: Vec<BooksImportMigrationRequestAccountsItem>) -> Self {
        self.accounts = Some(value);
        self
    }

    pub fn partners(mut self, value: Vec<BooksImportMigrationRequestPartnersItem>) -> Self {
        self.partners = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<BooksImportMigrationRequestItemsItem>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn opening_balances(mut self, value: BooksImportMigrationRequestOpeningBalances) -> Self {
        self.opening_balances = Some(value);
        self
    }

    pub fn journal(mut self, value: Vec<BooksImportMigrationRequestJournalItem>) -> Self {
        self.journal = Some(value);
        self
    }

    pub fn open_receivables(
        mut self,
        value: Vec<BooksImportMigrationRequestOpenReceivablesItem>,
    ) -> Self {
        self.open_receivables = Some(value);
        self
    }

    pub fn open_payables(
        mut self,
        value: Vec<BooksImportMigrationRequestOpenPayablesItem>,
    ) -> Self {
        self.open_payables = Some(value);
        self
    }

    pub fn asset_groups(mut self, value: Vec<BooksImportMigrationRequestAssetGroupsItem>) -> Self {
        self.asset_groups = Some(value);
        self
    }

    pub fn fixed_assets(mut self, value: Vec<BooksImportMigrationRequestFixedAssetsItem>) -> Self {
        self.fixed_assets = Some(value);
        self
    }

    pub fn stock(mut self, value: Vec<BooksImportMigrationRequestStockItem>) -> Self {
        self.stock = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BooksImportMigrationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cutover_date`](BooksImportMigrationRequestBuilder::cutover_date)
    pub fn build(self) -> Result<BooksImportMigrationRequest, BuildError> {
        Ok(BooksImportMigrationRequest {
            cutover_date: self
                .cutover_date
                .ok_or_else(|| BuildError::missing_field("cutover_date"))?,
            source: self.source,
            accounts: self.accounts,
            partners: self.partners,
            items: self.items,
            opening_balances: self.opening_balances,
            journal: self.journal,
            open_receivables: self.open_receivables,
            open_payables: self.open_payables,
            asset_groups: self.asset_groups,
            fixed_assets: self.fixed_assets,
            stock: self.stock,
        })
    }
}

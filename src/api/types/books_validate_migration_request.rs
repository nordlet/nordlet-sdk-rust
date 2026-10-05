pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BooksValidateMigrationRequest {
    #[serde(rename = "cutoverDate")]
    #[serde(default)]
    pub cutover_date: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<BooksValidateMigrationRequestAccountsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partners: Option<Vec<BooksValidateMigrationRequestPartnersItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<BooksValidateMigrationRequestItemsItem>>,
    #[serde(rename = "openingBalances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opening_balances: Option<BooksValidateMigrationRequestOpeningBalances>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal: Option<Vec<BooksValidateMigrationRequestJournalItem>>,
    #[serde(rename = "openReceivables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_receivables: Option<Vec<BooksValidateMigrationRequestOpenReceivablesItem>>,
    #[serde(rename = "openPayables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_payables: Option<Vec<BooksValidateMigrationRequestOpenPayablesItem>>,
    #[serde(rename = "assetGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_groups: Option<Vec<BooksValidateMigrationRequestAssetGroupsItem>>,
    #[serde(rename = "fixedAssets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_assets: Option<Vec<BooksValidateMigrationRequestFixedAssetsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock: Option<Vec<BooksValidateMigrationRequestStockItem>>,
}

impl BooksValidateMigrationRequest {
    pub fn builder() -> BooksValidateMigrationRequestBuilder {
        <BooksValidateMigrationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BooksValidateMigrationRequestBuilder {
    cutover_date: Option<NaiveDate>,
    source: Option<String>,
    accounts: Option<Vec<BooksValidateMigrationRequestAccountsItem>>,
    partners: Option<Vec<BooksValidateMigrationRequestPartnersItem>>,
    items: Option<Vec<BooksValidateMigrationRequestItemsItem>>,
    opening_balances: Option<BooksValidateMigrationRequestOpeningBalances>,
    journal: Option<Vec<BooksValidateMigrationRequestJournalItem>>,
    open_receivables: Option<Vec<BooksValidateMigrationRequestOpenReceivablesItem>>,
    open_payables: Option<Vec<BooksValidateMigrationRequestOpenPayablesItem>>,
    asset_groups: Option<Vec<BooksValidateMigrationRequestAssetGroupsItem>>,
    fixed_assets: Option<Vec<BooksValidateMigrationRequestFixedAssetsItem>>,
    stock: Option<Vec<BooksValidateMigrationRequestStockItem>>,
}

impl BooksValidateMigrationRequestBuilder {
    pub fn cutover_date(mut self, value: NaiveDate) -> Self {
        self.cutover_date = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn accounts(mut self, value: Vec<BooksValidateMigrationRequestAccountsItem>) -> Self {
        self.accounts = Some(value);
        self
    }

    pub fn partners(mut self, value: Vec<BooksValidateMigrationRequestPartnersItem>) -> Self {
        self.partners = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<BooksValidateMigrationRequestItemsItem>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn opening_balances(mut self, value: BooksValidateMigrationRequestOpeningBalances) -> Self {
        self.opening_balances = Some(value);
        self
    }

    pub fn journal(mut self, value: Vec<BooksValidateMigrationRequestJournalItem>) -> Self {
        self.journal = Some(value);
        self
    }

    pub fn open_receivables(
        mut self,
        value: Vec<BooksValidateMigrationRequestOpenReceivablesItem>,
    ) -> Self {
        self.open_receivables = Some(value);
        self
    }

    pub fn open_payables(
        mut self,
        value: Vec<BooksValidateMigrationRequestOpenPayablesItem>,
    ) -> Self {
        self.open_payables = Some(value);
        self
    }

    pub fn asset_groups(
        mut self,
        value: Vec<BooksValidateMigrationRequestAssetGroupsItem>,
    ) -> Self {
        self.asset_groups = Some(value);
        self
    }

    pub fn fixed_assets(
        mut self,
        value: Vec<BooksValidateMigrationRequestFixedAssetsItem>,
    ) -> Self {
        self.fixed_assets = Some(value);
        self
    }

    pub fn stock(mut self, value: Vec<BooksValidateMigrationRequestStockItem>) -> Self {
        self.stock = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BooksValidateMigrationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cutover_date`](BooksValidateMigrationRequestBuilder::cutover_date)
    pub fn build(self) -> Result<BooksValidateMigrationRequest, BuildError> {
        Ok(BooksValidateMigrationRequest {
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

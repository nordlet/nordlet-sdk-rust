pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1MigrationBooksImportRequest {
    #[serde(rename = "cutoverDate")]
    #[serde(default)]
    pub cutover_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<PostV1MigrationBooksImportRequestAccountsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partners: Option<Vec<PostV1MigrationBooksImportRequestPartnersItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PostV1MigrationBooksImportRequestItemsItem>>,
    #[serde(rename = "openingBalances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opening_balances: Option<PostV1MigrationBooksImportRequestOpeningBalances>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal: Option<Vec<PostV1MigrationBooksImportRequestJournalItem>>,
    #[serde(rename = "openReceivables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_receivables: Option<Vec<PostV1MigrationBooksImportRequestOpenReceivablesItem>>,
    #[serde(rename = "openPayables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_payables: Option<Vec<PostV1MigrationBooksImportRequestOpenPayablesItem>>,
    #[serde(rename = "assetGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_groups: Option<Vec<PostV1MigrationBooksImportRequestAssetGroupsItem>>,
    #[serde(rename = "fixedAssets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_assets: Option<Vec<PostV1MigrationBooksImportRequestFixedAssetsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock: Option<Vec<PostV1MigrationBooksImportRequestStockItem>>,
}

impl PostV1MigrationBooksImportRequest {
    pub fn builder() -> PostV1MigrationBooksImportRequestBuilder {
        <PostV1MigrationBooksImportRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1MigrationBooksImportRequestBuilder {
    cutover_date: Option<String>,
    source: Option<String>,
    accounts: Option<Vec<PostV1MigrationBooksImportRequestAccountsItem>>,
    partners: Option<Vec<PostV1MigrationBooksImportRequestPartnersItem>>,
    items: Option<Vec<PostV1MigrationBooksImportRequestItemsItem>>,
    opening_balances: Option<PostV1MigrationBooksImportRequestOpeningBalances>,
    journal: Option<Vec<PostV1MigrationBooksImportRequestJournalItem>>,
    open_receivables: Option<Vec<PostV1MigrationBooksImportRequestOpenReceivablesItem>>,
    open_payables: Option<Vec<PostV1MigrationBooksImportRequestOpenPayablesItem>>,
    asset_groups: Option<Vec<PostV1MigrationBooksImportRequestAssetGroupsItem>>,
    fixed_assets: Option<Vec<PostV1MigrationBooksImportRequestFixedAssetsItem>>,
    stock: Option<Vec<PostV1MigrationBooksImportRequestStockItem>>,
}

impl PostV1MigrationBooksImportRequestBuilder {
    pub fn cutover_date(mut self, value: impl Into<String>) -> Self {
        self.cutover_date = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn accounts(mut self, value: Vec<PostV1MigrationBooksImportRequestAccountsItem>) -> Self {
        self.accounts = Some(value);
        self
    }

    pub fn partners(mut self, value: Vec<PostV1MigrationBooksImportRequestPartnersItem>) -> Self {
        self.partners = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<PostV1MigrationBooksImportRequestItemsItem>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn opening_balances(
        mut self,
        value: PostV1MigrationBooksImportRequestOpeningBalances,
    ) -> Self {
        self.opening_balances = Some(value);
        self
    }

    pub fn journal(mut self, value: Vec<PostV1MigrationBooksImportRequestJournalItem>) -> Self {
        self.journal = Some(value);
        self
    }

    pub fn open_receivables(
        mut self,
        value: Vec<PostV1MigrationBooksImportRequestOpenReceivablesItem>,
    ) -> Self {
        self.open_receivables = Some(value);
        self
    }

    pub fn open_payables(
        mut self,
        value: Vec<PostV1MigrationBooksImportRequestOpenPayablesItem>,
    ) -> Self {
        self.open_payables = Some(value);
        self
    }

    pub fn asset_groups(
        mut self,
        value: Vec<PostV1MigrationBooksImportRequestAssetGroupsItem>,
    ) -> Self {
        self.asset_groups = Some(value);
        self
    }

    pub fn fixed_assets(
        mut self,
        value: Vec<PostV1MigrationBooksImportRequestFixedAssetsItem>,
    ) -> Self {
        self.fixed_assets = Some(value);
        self
    }

    pub fn stock(mut self, value: Vec<PostV1MigrationBooksImportRequestStockItem>) -> Self {
        self.stock = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1MigrationBooksImportRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cutover_date`](PostV1MigrationBooksImportRequestBuilder::cutover_date)
    pub fn build(self) -> Result<PostV1MigrationBooksImportRequest, BuildError> {
        Ok(PostV1MigrationBooksImportRequest {
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

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1MigrationBooksValidateRequest {
    #[serde(rename = "cutoverDate")]
    #[serde(default)]
    pub cutover_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<PostV1MigrationBooksValidateRequestAccountsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partners: Option<Vec<PostV1MigrationBooksValidateRequestPartnersItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PostV1MigrationBooksValidateRequestItemsItem>>,
    #[serde(rename = "openingBalances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opening_balances: Option<PostV1MigrationBooksValidateRequestOpeningBalances>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal: Option<Vec<PostV1MigrationBooksValidateRequestJournalItem>>,
    #[serde(rename = "openReceivables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_receivables: Option<Vec<PostV1MigrationBooksValidateRequestOpenReceivablesItem>>,
    #[serde(rename = "openPayables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_payables: Option<Vec<PostV1MigrationBooksValidateRequestOpenPayablesItem>>,
    #[serde(rename = "assetGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_groups: Option<Vec<PostV1MigrationBooksValidateRequestAssetGroupsItem>>,
    #[serde(rename = "fixedAssets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_assets: Option<Vec<PostV1MigrationBooksValidateRequestFixedAssetsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock: Option<Vec<PostV1MigrationBooksValidateRequestStockItem>>,
}

impl PostV1MigrationBooksValidateRequest {
    pub fn builder() -> PostV1MigrationBooksValidateRequestBuilder {
        <PostV1MigrationBooksValidateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1MigrationBooksValidateRequestBuilder {
    cutover_date: Option<String>,
    source: Option<String>,
    accounts: Option<Vec<PostV1MigrationBooksValidateRequestAccountsItem>>,
    partners: Option<Vec<PostV1MigrationBooksValidateRequestPartnersItem>>,
    items: Option<Vec<PostV1MigrationBooksValidateRequestItemsItem>>,
    opening_balances: Option<PostV1MigrationBooksValidateRequestOpeningBalances>,
    journal: Option<Vec<PostV1MigrationBooksValidateRequestJournalItem>>,
    open_receivables: Option<Vec<PostV1MigrationBooksValidateRequestOpenReceivablesItem>>,
    open_payables: Option<Vec<PostV1MigrationBooksValidateRequestOpenPayablesItem>>,
    asset_groups: Option<Vec<PostV1MigrationBooksValidateRequestAssetGroupsItem>>,
    fixed_assets: Option<Vec<PostV1MigrationBooksValidateRequestFixedAssetsItem>>,
    stock: Option<Vec<PostV1MigrationBooksValidateRequestStockItem>>,
}

impl PostV1MigrationBooksValidateRequestBuilder {
    pub fn cutover_date(mut self, value: impl Into<String>) -> Self {
        self.cutover_date = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn accounts(mut self, value: Vec<PostV1MigrationBooksValidateRequestAccountsItem>) -> Self {
        self.accounts = Some(value);
        self
    }

    pub fn partners(mut self, value: Vec<PostV1MigrationBooksValidateRequestPartnersItem>) -> Self {
        self.partners = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<PostV1MigrationBooksValidateRequestItemsItem>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn opening_balances(
        mut self,
        value: PostV1MigrationBooksValidateRequestOpeningBalances,
    ) -> Self {
        self.opening_balances = Some(value);
        self
    }

    pub fn journal(mut self, value: Vec<PostV1MigrationBooksValidateRequestJournalItem>) -> Self {
        self.journal = Some(value);
        self
    }

    pub fn open_receivables(
        mut self,
        value: Vec<PostV1MigrationBooksValidateRequestOpenReceivablesItem>,
    ) -> Self {
        self.open_receivables = Some(value);
        self
    }

    pub fn open_payables(
        mut self,
        value: Vec<PostV1MigrationBooksValidateRequestOpenPayablesItem>,
    ) -> Self {
        self.open_payables = Some(value);
        self
    }

    pub fn asset_groups(
        mut self,
        value: Vec<PostV1MigrationBooksValidateRequestAssetGroupsItem>,
    ) -> Self {
        self.asset_groups = Some(value);
        self
    }

    pub fn fixed_assets(
        mut self,
        value: Vec<PostV1MigrationBooksValidateRequestFixedAssetsItem>,
    ) -> Self {
        self.fixed_assets = Some(value);
        self
    }

    pub fn stock(mut self, value: Vec<PostV1MigrationBooksValidateRequestStockItem>) -> Self {
        self.stock = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1MigrationBooksValidateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cutover_date`](PostV1MigrationBooksValidateRequestBuilder::cutover_date)
    pub fn build(self) -> Result<PostV1MigrationBooksValidateRequest, BuildError> {
        Ok(PostV1MigrationBooksValidateRequest {
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

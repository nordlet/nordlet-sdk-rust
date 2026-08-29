pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationIntercompanyReportResponseDirectionsItem {
    #[serde(rename = "sellerCompanyId")]
    #[serde(default)]
    pub seller_company_id: String,
    #[serde(rename = "sellerName")]
    #[serde(default)]
    pub seller_name: String,
    #[serde(rename = "buyerCompanyId")]
    #[serde(default)]
    pub buyer_company_id: String,
    #[serde(rename = "buyerName")]
    #[serde(default)]
    pub buyer_name: String,
    #[serde(default)]
    pub documents: Vec<PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItem>,
    #[serde(rename = "unmatchedPurchases")]
    #[serde(default)]
    pub unmatched_purchases:
        Vec<PostV1ConsolidationIntercompanyReportResponseDirectionsItemUnmatchedPurchasesItem>,
    #[serde(default)]
    pub totals: Vec<PostV1ConsolidationIntercompanyReportResponseDirectionsItemTotalsItem>,
}

impl PostV1ConsolidationIntercompanyReportResponseDirectionsItem {
    pub fn builder() -> PostV1ConsolidationIntercompanyReportResponseDirectionsItemBuilder {
        <PostV1ConsolidationIntercompanyReportResponseDirectionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationIntercompanyReportResponseDirectionsItemBuilder {
    seller_company_id: Option<String>,
    seller_name: Option<String>,
    buyer_company_id: Option<String>,
    buyer_name: Option<String>,
    documents:
        Option<Vec<PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItem>>,
    unmatched_purchases: Option<
        Vec<PostV1ConsolidationIntercompanyReportResponseDirectionsItemUnmatchedPurchasesItem>,
    >,
    totals: Option<Vec<PostV1ConsolidationIntercompanyReportResponseDirectionsItemTotalsItem>>,
}

impl PostV1ConsolidationIntercompanyReportResponseDirectionsItemBuilder {
    pub fn seller_company_id(mut self, value: impl Into<String>) -> Self {
        self.seller_company_id = Some(value.into());
        self
    }

    pub fn seller_name(mut self, value: impl Into<String>) -> Self {
        self.seller_name = Some(value.into());
        self
    }

    pub fn buyer_company_id(mut self, value: impl Into<String>) -> Self {
        self.buyer_company_id = Some(value.into());
        self
    }

    pub fn buyer_name(mut self, value: impl Into<String>) -> Self {
        self.buyer_name = Some(value.into());
        self
    }

    pub fn documents(
        mut self,
        value: Vec<PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItem>,
    ) -> Self {
        self.documents = Some(value);
        self
    }

    pub fn unmatched_purchases(
        mut self,
        value: Vec<
            PostV1ConsolidationIntercompanyReportResponseDirectionsItemUnmatchedPurchasesItem,
        >,
    ) -> Self {
        self.unmatched_purchases = Some(value);
        self
    }

    pub fn totals(
        mut self,
        value: Vec<PostV1ConsolidationIntercompanyReportResponseDirectionsItemTotalsItem>,
    ) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationIntercompanyReportResponseDirectionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`seller_company_id`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemBuilder::seller_company_id)
    /// - [`seller_name`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemBuilder::seller_name)
    /// - [`buyer_company_id`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemBuilder::buyer_company_id)
    /// - [`buyer_name`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemBuilder::buyer_name)
    /// - [`documents`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemBuilder::documents)
    /// - [`unmatched_purchases`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemBuilder::unmatched_purchases)
    /// - [`totals`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemBuilder::totals)
    pub fn build(
        self,
    ) -> Result<PostV1ConsolidationIntercompanyReportResponseDirectionsItem, BuildError> {
        Ok(
            PostV1ConsolidationIntercompanyReportResponseDirectionsItem {
                seller_company_id: self
                    .seller_company_id
                    .ok_or_else(|| BuildError::missing_field("seller_company_id"))?,
                seller_name: self
                    .seller_name
                    .ok_or_else(|| BuildError::missing_field("seller_name"))?,
                buyer_company_id: self
                    .buyer_company_id
                    .ok_or_else(|| BuildError::missing_field("buyer_company_id"))?,
                buyer_name: self
                    .buyer_name
                    .ok_or_else(|| BuildError::missing_field("buyer_name"))?,
                documents: self
                    .documents
                    .ok_or_else(|| BuildError::missing_field("documents"))?,
                unmatched_purchases: self
                    .unmatched_purchases
                    .ok_or_else(|| BuildError::missing_field("unmatched_purchases"))?,
                totals: self
                    .totals
                    .ok_or_else(|| BuildError::missing_field("totals"))?,
            },
        )
    }
}

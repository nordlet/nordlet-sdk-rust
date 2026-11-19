pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsDeFranchiseTaxComputeDeclarationsRequest {
    #[serde(rename = "authorizedShares")]
    #[serde(default)]
    pub authorized_shares: Vec<UsDeFranchiseTaxComputeDeclarationsRequestAuthorizedSharesItem>,
    #[serde(rename = "issuedShares")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_shares: Option<i64>,
    #[serde(rename = "totalGrossAssets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_gross_assets: Option<String>,
    #[serde(rename = "largeCorporateFiler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_corporate_filer: Option<bool>,
}

impl UsDeFranchiseTaxComputeDeclarationsRequest {
    pub fn builder() -> UsDeFranchiseTaxComputeDeclarationsRequestBuilder {
        <UsDeFranchiseTaxComputeDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsDeFranchiseTaxComputeDeclarationsRequestBuilder {
    authorized_shares: Option<Vec<UsDeFranchiseTaxComputeDeclarationsRequestAuthorizedSharesItem>>,
    issued_shares: Option<i64>,
    total_gross_assets: Option<String>,
    large_corporate_filer: Option<bool>,
}

impl UsDeFranchiseTaxComputeDeclarationsRequestBuilder {
    pub fn authorized_shares(
        mut self,
        value: Vec<UsDeFranchiseTaxComputeDeclarationsRequestAuthorizedSharesItem>,
    ) -> Self {
        self.authorized_shares = Some(value);
        self
    }

    pub fn issued_shares(mut self, value: i64) -> Self {
        self.issued_shares = Some(value);
        self
    }

    pub fn total_gross_assets(mut self, value: impl Into<String>) -> Self {
        self.total_gross_assets = Some(value.into());
        self
    }

    pub fn large_corporate_filer(mut self, value: bool) -> Self {
        self.large_corporate_filer = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsDeFranchiseTaxComputeDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`authorized_shares`](UsDeFranchiseTaxComputeDeclarationsRequestBuilder::authorized_shares)
    pub fn build(self) -> Result<UsDeFranchiseTaxComputeDeclarationsRequest, BuildError> {
        Ok(UsDeFranchiseTaxComputeDeclarationsRequest {
            authorized_shares: self
                .authorized_shares
                .ok_or_else(|| BuildError::missing_field("authorized_shares"))?,
            issued_shares: self.issued_shares,
            total_gross_assets: self.total_gross_assets,
            large_corporate_filer: self.large_corporate_filer,
        })
    }
}

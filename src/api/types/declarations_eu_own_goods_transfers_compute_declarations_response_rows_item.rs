pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuOwnGoodsTransfersComputeDeclarationsResponseRowsItem {
    #[serde(rename = "destinationCountryCode")]
    #[serde(default)]
    pub destination_country_code: String,
    #[serde(rename = "dispatchCountryCode")]
    #[serde(default)]
    pub dispatch_country_code: String,
    #[serde(rename = "taxableAmount")]
    #[serde(default)]
    pub taxable_amount: String,
    #[serde(default)]
    pub transfers: i64,
}

impl EuOwnGoodsTransfersComputeDeclarationsResponseRowsItem {
    pub fn builder() -> EuOwnGoodsTransfersComputeDeclarationsResponseRowsItemBuilder {
        <EuOwnGoodsTransfersComputeDeclarationsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuOwnGoodsTransfersComputeDeclarationsResponseRowsItemBuilder {
    destination_country_code: Option<String>,
    dispatch_country_code: Option<String>,
    taxable_amount: Option<String>,
    transfers: Option<i64>,
}

impl EuOwnGoodsTransfersComputeDeclarationsResponseRowsItemBuilder {
    pub fn destination_country_code(mut self, value: impl Into<String>) -> Self {
        self.destination_country_code = Some(value.into());
        self
    }

    pub fn dispatch_country_code(mut self, value: impl Into<String>) -> Self {
        self.dispatch_country_code = Some(value.into());
        self
    }

    pub fn taxable_amount(mut self, value: impl Into<String>) -> Self {
        self.taxable_amount = Some(value.into());
        self
    }

    pub fn transfers(mut self, value: i64) -> Self {
        self.transfers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EuOwnGoodsTransfersComputeDeclarationsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`destination_country_code`](EuOwnGoodsTransfersComputeDeclarationsResponseRowsItemBuilder::destination_country_code)
    /// - [`dispatch_country_code`](EuOwnGoodsTransfersComputeDeclarationsResponseRowsItemBuilder::dispatch_country_code)
    /// - [`taxable_amount`](EuOwnGoodsTransfersComputeDeclarationsResponseRowsItemBuilder::taxable_amount)
    /// - [`transfers`](EuOwnGoodsTransfersComputeDeclarationsResponseRowsItemBuilder::transfers)
    pub fn build(
        self,
    ) -> Result<EuOwnGoodsTransfersComputeDeclarationsResponseRowsItem, BuildError> {
        Ok(EuOwnGoodsTransfersComputeDeclarationsResponseRowsItem {
            destination_country_code: self
                .destination_country_code
                .ok_or_else(|| BuildError::missing_field("destination_country_code"))?,
            dispatch_country_code: self
                .dispatch_country_code
                .ok_or_else(|| BuildError::missing_field("dispatch_country_code"))?,
            taxable_amount: self
                .taxable_amount
                .ok_or_else(|| BuildError::missing_field("taxable_amount"))?,
            transfers: self
                .transfers
                .ok_or_else(|| BuildError::missing_field("transfers"))?,
        })
    }
}

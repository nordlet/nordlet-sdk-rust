pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PartnerBalancesReportsResponseRowsItem {
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(rename = "partnerName")]
    #[serde(default)]
    pub partner_name: String,
    #[serde(default)]
    pub receivable: String,
    #[serde(default)]
    pub payable: String,
    #[serde(default)]
    pub net: String,
}

impl PartnerBalancesReportsResponseRowsItem {
    pub fn builder() -> PartnerBalancesReportsResponseRowsItemBuilder {
        <PartnerBalancesReportsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PartnerBalancesReportsResponseRowsItemBuilder {
    partner_id: Option<String>,
    partner_name: Option<String>,
    receivable: Option<String>,
    payable: Option<String>,
    net: Option<String>,
}

impl PartnerBalancesReportsResponseRowsItemBuilder {
    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn partner_name(mut self, value: impl Into<String>) -> Self {
        self.partner_name = Some(value.into());
        self
    }

    pub fn receivable(mut self, value: impl Into<String>) -> Self {
        self.receivable = Some(value.into());
        self
    }

    pub fn payable(mut self, value: impl Into<String>) -> Self {
        self.payable = Some(value.into());
        self
    }

    pub fn net(mut self, value: impl Into<String>) -> Self {
        self.net = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PartnerBalancesReportsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`partner_id`](PartnerBalancesReportsResponseRowsItemBuilder::partner_id)
    /// - [`partner_name`](PartnerBalancesReportsResponseRowsItemBuilder::partner_name)
    /// - [`receivable`](PartnerBalancesReportsResponseRowsItemBuilder::receivable)
    /// - [`payable`](PartnerBalancesReportsResponseRowsItemBuilder::payable)
    /// - [`net`](PartnerBalancesReportsResponseRowsItemBuilder::net)
    pub fn build(self) -> Result<PartnerBalancesReportsResponseRowsItem, BuildError> {
        Ok(PartnerBalancesReportsResponseRowsItem {
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            partner_name: self
                .partner_name
                .ok_or_else(|| BuildError::missing_field("partner_name"))?,
            receivable: self
                .receivable
                .ok_or_else(|| BuildError::missing_field("receivable"))?,
            payable: self
                .payable
                .ok_or_else(|| BuildError::missing_field("payable"))?,
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
        })
    }
}

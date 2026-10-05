pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IntercompanyLinksSetConsolidationRequest {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(rename = "counterpartyCompanyId")]
    #[serde(default)]
    pub counterparty_company_id: String,
}

impl IntercompanyLinksSetConsolidationRequest {
    pub fn builder() -> IntercompanyLinksSetConsolidationRequestBuilder {
        <IntercompanyLinksSetConsolidationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IntercompanyLinksSetConsolidationRequestBuilder {
    group_id: Option<String>,
    partner_id: Option<String>,
    counterparty_company_id: Option<String>,
}

impl IntercompanyLinksSetConsolidationRequestBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn counterparty_company_id(mut self, value: impl Into<String>) -> Self {
        self.counterparty_company_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IntercompanyLinksSetConsolidationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](IntercompanyLinksSetConsolidationRequestBuilder::group_id)
    /// - [`partner_id`](IntercompanyLinksSetConsolidationRequestBuilder::partner_id)
    /// - [`counterparty_company_id`](IntercompanyLinksSetConsolidationRequestBuilder::counterparty_company_id)
    pub fn build(self) -> Result<IntercompanyLinksSetConsolidationRequest, BuildError> {
        Ok(IntercompanyLinksSetConsolidationRequest {
            group_id: self
                .group_id
                .ok_or_else(|| BuildError::missing_field("group_id"))?,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            counterparty_company_id: self
                .counterparty_company_id
                .ok_or_else(|| BuildError::missing_field("counterparty_company_id"))?,
        })
    }
}

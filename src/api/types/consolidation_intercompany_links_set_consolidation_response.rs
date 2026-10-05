pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IntercompanyLinksSetConsolidationResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(rename = "companyId")]
    #[serde(default)]
    pub company_id: String,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(rename = "counterpartyCompanyId")]
    #[serde(default)]
    pub counterparty_company_id: String,
}

impl IntercompanyLinksSetConsolidationResponse {
    pub fn builder() -> IntercompanyLinksSetConsolidationResponseBuilder {
        <IntercompanyLinksSetConsolidationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IntercompanyLinksSetConsolidationResponseBuilder {
    id: Option<String>,
    group_id: Option<String>,
    company_id: Option<String>,
    partner_id: Option<String>,
    counterparty_company_id: Option<String>,
}

impl IntercompanyLinksSetConsolidationResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`IntercompanyLinksSetConsolidationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](IntercompanyLinksSetConsolidationResponseBuilder::id)
    /// - [`group_id`](IntercompanyLinksSetConsolidationResponseBuilder::group_id)
    /// - [`company_id`](IntercompanyLinksSetConsolidationResponseBuilder::company_id)
    /// - [`partner_id`](IntercompanyLinksSetConsolidationResponseBuilder::partner_id)
    /// - [`counterparty_company_id`](IntercompanyLinksSetConsolidationResponseBuilder::counterparty_company_id)
    pub fn build(self) -> Result<IntercompanyLinksSetConsolidationResponse, BuildError> {
        Ok(IntercompanyLinksSetConsolidationResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            group_id: self
                .group_id
                .ok_or_else(|| BuildError::missing_field("group_id"))?,
            company_id: self
                .company_id
                .ok_or_else(|| BuildError::missing_field("company_id"))?,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            counterparty_company_id: self
                .counterparty_company_id
                .ok_or_else(|| BuildError::missing_field("counterparty_company_id"))?,
        })
    }
}

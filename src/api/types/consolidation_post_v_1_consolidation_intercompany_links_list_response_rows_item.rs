pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationIntercompanyLinksListResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "companyId")]
    #[serde(default)]
    pub company_id: String,
    #[serde(rename = "companyName")]
    #[serde(default)]
    pub company_name: String,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(rename = "partnerName")]
    #[serde(default)]
    pub partner_name: String,
    #[serde(rename = "counterpartyCompanyId")]
    #[serde(default)]
    pub counterparty_company_id: String,
    #[serde(rename = "counterpartyCompanyName")]
    #[serde(default)]
    pub counterparty_company_name: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1ConsolidationIntercompanyLinksListResponseRowsItem {
    pub fn builder() -> PostV1ConsolidationIntercompanyLinksListResponseRowsItemBuilder {
        <PostV1ConsolidationIntercompanyLinksListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationIntercompanyLinksListResponseRowsItemBuilder {
    id: Option<String>,
    company_id: Option<String>,
    company_name: Option<String>,
    partner_id: Option<String>,
    partner_name: Option<String>,
    counterparty_company_id: Option<String>,
    counterparty_company_name: Option<String>,
    created_at: Option<String>,
}

impl PostV1ConsolidationIntercompanyLinksListResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    pub fn company_name(mut self, value: impl Into<String>) -> Self {
        self.company_name = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn partner_name(mut self, value: impl Into<String>) -> Self {
        self.partner_name = Some(value.into());
        self
    }

    pub fn counterparty_company_id(mut self, value: impl Into<String>) -> Self {
        self.counterparty_company_id = Some(value.into());
        self
    }

    pub fn counterparty_company_name(mut self, value: impl Into<String>) -> Self {
        self.counterparty_company_name = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationIntercompanyLinksListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ConsolidationIntercompanyLinksListResponseRowsItemBuilder::id)
    /// - [`company_id`](PostV1ConsolidationIntercompanyLinksListResponseRowsItemBuilder::company_id)
    /// - [`company_name`](PostV1ConsolidationIntercompanyLinksListResponseRowsItemBuilder::company_name)
    /// - [`partner_id`](PostV1ConsolidationIntercompanyLinksListResponseRowsItemBuilder::partner_id)
    /// - [`partner_name`](PostV1ConsolidationIntercompanyLinksListResponseRowsItemBuilder::partner_name)
    /// - [`counterparty_company_id`](PostV1ConsolidationIntercompanyLinksListResponseRowsItemBuilder::counterparty_company_id)
    /// - [`counterparty_company_name`](PostV1ConsolidationIntercompanyLinksListResponseRowsItemBuilder::counterparty_company_name)
    /// - [`created_at`](PostV1ConsolidationIntercompanyLinksListResponseRowsItemBuilder::created_at)
    pub fn build(
        self,
    ) -> Result<PostV1ConsolidationIntercompanyLinksListResponseRowsItem, BuildError> {
        Ok(PostV1ConsolidationIntercompanyLinksListResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            company_id: self
                .company_id
                .ok_or_else(|| BuildError::missing_field("company_id"))?,
            company_name: self
                .company_name
                .ok_or_else(|| BuildError::missing_field("company_name"))?,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            partner_name: self
                .partner_name
                .ok_or_else(|| BuildError::missing_field("partner_name"))?,
            counterparty_company_id: self
                .counterparty_company_id
                .ok_or_else(|| BuildError::missing_field("counterparty_company_id"))?,
            counterparty_company_name: self
                .counterparty_company_name
                .ok_or_else(|| BuildError::missing_field("counterparty_company_name"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationIntercompanyLinksSetResponse {
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

impl PostV1ConsolidationIntercompanyLinksSetResponse {
    pub fn builder() -> PostV1ConsolidationIntercompanyLinksSetResponseBuilder {
        <PostV1ConsolidationIntercompanyLinksSetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationIntercompanyLinksSetResponseBuilder {
    id: Option<String>,
    group_id: Option<String>,
    company_id: Option<String>,
    partner_id: Option<String>,
    counterparty_company_id: Option<String>,
}

impl PostV1ConsolidationIntercompanyLinksSetResponseBuilder {
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

    /// Consumes the builder and constructs a [`PostV1ConsolidationIntercompanyLinksSetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ConsolidationIntercompanyLinksSetResponseBuilder::id)
    /// - [`group_id`](PostV1ConsolidationIntercompanyLinksSetResponseBuilder::group_id)
    /// - [`company_id`](PostV1ConsolidationIntercompanyLinksSetResponseBuilder::company_id)
    /// - [`partner_id`](PostV1ConsolidationIntercompanyLinksSetResponseBuilder::partner_id)
    /// - [`counterparty_company_id`](PostV1ConsolidationIntercompanyLinksSetResponseBuilder::counterparty_company_id)
    pub fn build(self) -> Result<PostV1ConsolidationIntercompanyLinksSetResponse, BuildError> {
        Ok(PostV1ConsolidationIntercompanyLinksSetResponse {
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

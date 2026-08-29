pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationIntercompanyLinksSetRequest {
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

impl PostV1ConsolidationIntercompanyLinksSetRequest {
    pub fn builder() -> PostV1ConsolidationIntercompanyLinksSetRequestBuilder {
        <PostV1ConsolidationIntercompanyLinksSetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationIntercompanyLinksSetRequestBuilder {
    group_id: Option<String>,
    partner_id: Option<String>,
    counterparty_company_id: Option<String>,
}

impl PostV1ConsolidationIntercompanyLinksSetRequestBuilder {
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

    /// Consumes the builder and constructs a [`PostV1ConsolidationIntercompanyLinksSetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](PostV1ConsolidationIntercompanyLinksSetRequestBuilder::group_id)
    /// - [`partner_id`](PostV1ConsolidationIntercompanyLinksSetRequestBuilder::partner_id)
    /// - [`counterparty_company_id`](PostV1ConsolidationIntercompanyLinksSetRequestBuilder::counterparty_company_id)
    pub fn build(self) -> Result<PostV1ConsolidationIntercompanyLinksSetRequest, BuildError> {
        Ok(PostV1ConsolidationIntercompanyLinksSetRequest {
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

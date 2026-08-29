pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationIntercompanyCandidatesResponseRowsItem {
    #[serde(rename = "memberCompanyId")]
    #[serde(default)]
    pub member_company_id: String,
    #[serde(rename = "memberName")]
    #[serde(default)]
    pub member_name: String,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(rename = "partnerName")]
    #[serde(default)]
    pub partner_name: String,
    #[serde(rename = "partnerCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_code: Option<String>,
    #[serde(rename = "matchesCompanyId")]
    #[serde(default)]
    pub matches_company_id: String,
    #[serde(rename = "matchesCompanyName")]
    #[serde(default)]
    pub matches_company_name: String,
    #[serde(rename = "matchedOn")]
    pub matched_on: PostV1ConsolidationIntercompanyCandidatesResponseRowsItemMatchedOn,
    #[serde(rename = "linkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<String>,
}

impl PostV1ConsolidationIntercompanyCandidatesResponseRowsItem {
    pub fn builder() -> PostV1ConsolidationIntercompanyCandidatesResponseRowsItemBuilder {
        <PostV1ConsolidationIntercompanyCandidatesResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationIntercompanyCandidatesResponseRowsItemBuilder {
    member_company_id: Option<String>,
    member_name: Option<String>,
    partner_id: Option<String>,
    partner_name: Option<String>,
    partner_code: Option<String>,
    matches_company_id: Option<String>,
    matches_company_name: Option<String>,
    matched_on: Option<PostV1ConsolidationIntercompanyCandidatesResponseRowsItemMatchedOn>,
    link_id: Option<String>,
}

impl PostV1ConsolidationIntercompanyCandidatesResponseRowsItemBuilder {
    pub fn member_company_id(mut self, value: impl Into<String>) -> Self {
        self.member_company_id = Some(value.into());
        self
    }

    pub fn member_name(mut self, value: impl Into<String>) -> Self {
        self.member_name = Some(value.into());
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

    pub fn partner_code(mut self, value: impl Into<String>) -> Self {
        self.partner_code = Some(value.into());
        self
    }

    pub fn matches_company_id(mut self, value: impl Into<String>) -> Self {
        self.matches_company_id = Some(value.into());
        self
    }

    pub fn matches_company_name(mut self, value: impl Into<String>) -> Self {
        self.matches_company_name = Some(value.into());
        self
    }

    pub fn matched_on(
        mut self,
        value: PostV1ConsolidationIntercompanyCandidatesResponseRowsItemMatchedOn,
    ) -> Self {
        self.matched_on = Some(value);
        self
    }

    pub fn link_id(mut self, value: impl Into<String>) -> Self {
        self.link_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationIntercompanyCandidatesResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`member_company_id`](PostV1ConsolidationIntercompanyCandidatesResponseRowsItemBuilder::member_company_id)
    /// - [`member_name`](PostV1ConsolidationIntercompanyCandidatesResponseRowsItemBuilder::member_name)
    /// - [`partner_id`](PostV1ConsolidationIntercompanyCandidatesResponseRowsItemBuilder::partner_id)
    /// - [`partner_name`](PostV1ConsolidationIntercompanyCandidatesResponseRowsItemBuilder::partner_name)
    /// - [`matches_company_id`](PostV1ConsolidationIntercompanyCandidatesResponseRowsItemBuilder::matches_company_id)
    /// - [`matches_company_name`](PostV1ConsolidationIntercompanyCandidatesResponseRowsItemBuilder::matches_company_name)
    /// - [`matched_on`](PostV1ConsolidationIntercompanyCandidatesResponseRowsItemBuilder::matched_on)
    pub fn build(
        self,
    ) -> Result<PostV1ConsolidationIntercompanyCandidatesResponseRowsItem, BuildError> {
        Ok(PostV1ConsolidationIntercompanyCandidatesResponseRowsItem {
            member_company_id: self
                .member_company_id
                .ok_or_else(|| BuildError::missing_field("member_company_id"))?,
            member_name: self
                .member_name
                .ok_or_else(|| BuildError::missing_field("member_name"))?,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            partner_name: self
                .partner_name
                .ok_or_else(|| BuildError::missing_field("partner_name"))?,
            partner_code: self.partner_code,
            matches_company_id: self
                .matches_company_id
                .ok_or_else(|| BuildError::missing_field("matches_company_id"))?,
            matches_company_name: self
                .matches_company_name
                .ok_or_else(|| BuildError::missing_field("matches_company_name"))?,
            matched_on: self
                .matched_on
                .ok_or_else(|| BuildError::missing_field("matched_on"))?,
            link_id: self.link_id,
        })
    }
}

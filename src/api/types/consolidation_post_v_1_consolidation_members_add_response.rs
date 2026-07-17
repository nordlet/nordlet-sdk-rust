pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationMembersAddResponse {
    #[serde(rename = "memberCompanyId")]
    #[serde(default)]
    pub member_company_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "baseCurrency")]
    #[serde(default)]
    pub base_currency: String,
    #[serde(rename = "ownershipPercent")]
    #[serde(default)]
    pub ownership_percent: String,
    pub method: PostV1ConsolidationMembersAddResponseMethod,
}

impl PostV1ConsolidationMembersAddResponse {
    pub fn builder() -> PostV1ConsolidationMembersAddResponseBuilder {
        <PostV1ConsolidationMembersAddResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationMembersAddResponseBuilder {
    member_company_id: Option<String>,
    name: Option<String>,
    base_currency: Option<String>,
    ownership_percent: Option<String>,
    method: Option<PostV1ConsolidationMembersAddResponseMethod>,
}

impl PostV1ConsolidationMembersAddResponseBuilder {
    pub fn member_company_id(mut self, value: impl Into<String>) -> Self {
        self.member_company_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn base_currency(mut self, value: impl Into<String>) -> Self {
        self.base_currency = Some(value.into());
        self
    }

    pub fn ownership_percent(mut self, value: impl Into<String>) -> Self {
        self.ownership_percent = Some(value.into());
        self
    }

    pub fn method(mut self, value: PostV1ConsolidationMembersAddResponseMethod) -> Self {
        self.method = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationMembersAddResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`member_company_id`](PostV1ConsolidationMembersAddResponseBuilder::member_company_id)
    /// - [`name`](PostV1ConsolidationMembersAddResponseBuilder::name)
    /// - [`base_currency`](PostV1ConsolidationMembersAddResponseBuilder::base_currency)
    /// - [`ownership_percent`](PostV1ConsolidationMembersAddResponseBuilder::ownership_percent)
    /// - [`method`](PostV1ConsolidationMembersAddResponseBuilder::method)
    pub fn build(self) -> Result<PostV1ConsolidationMembersAddResponse, BuildError> {
        Ok(PostV1ConsolidationMembersAddResponse {
            member_company_id: self
                .member_company_id
                .ok_or_else(|| BuildError::missing_field("member_company_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            base_currency: self
                .base_currency
                .ok_or_else(|| BuildError::missing_field("base_currency"))?,
            ownership_percent: self
                .ownership_percent
                .ok_or_else(|| BuildError::missing_field("ownership_percent"))?,
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
        })
    }
}

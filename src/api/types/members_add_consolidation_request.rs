pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MembersAddConsolidationRequest {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(rename = "memberCompanyId")]
    #[serde(default)]
    pub member_company_id: String,
    #[serde(rename = "ownershipPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub ownership_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<MembersAddConsolidationRequestMethod>,
}

impl MembersAddConsolidationRequest {
    pub fn builder() -> MembersAddConsolidationRequestBuilder {
        <MembersAddConsolidationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembersAddConsolidationRequestBuilder {
    group_id: Option<String>,
    member_company_id: Option<String>,
    ownership_percent: Option<f64>,
    method: Option<MembersAddConsolidationRequestMethod>,
}

impl MembersAddConsolidationRequestBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn member_company_id(mut self, value: impl Into<String>) -> Self {
        self.member_company_id = Some(value.into());
        self
    }

    pub fn ownership_percent(mut self, value: f64) -> Self {
        self.ownership_percent = Some(value);
        self
    }

    pub fn method(mut self, value: MembersAddConsolidationRequestMethod) -> Self {
        self.method = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MembersAddConsolidationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](MembersAddConsolidationRequestBuilder::group_id)
    /// - [`member_company_id`](MembersAddConsolidationRequestBuilder::member_company_id)
    pub fn build(self) -> Result<MembersAddConsolidationRequest, BuildError> {
        Ok(MembersAddConsolidationRequest {
            group_id: self
                .group_id
                .ok_or_else(|| BuildError::missing_field("group_id"))?,
            member_company_id: self
                .member_company_id
                .ok_or_else(|| BuildError::missing_field("member_company_id"))?,
            ownership_percent: self.ownership_percent,
            method: self.method,
        })
    }
}

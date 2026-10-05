pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MembersRemoveConsolidationRequest {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(rename = "memberCompanyId")]
    #[serde(default)]
    pub member_company_id: String,
}

impl MembersRemoveConsolidationRequest {
    pub fn builder() -> MembersRemoveConsolidationRequestBuilder {
        <MembersRemoveConsolidationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembersRemoveConsolidationRequestBuilder {
    group_id: Option<String>,
    member_company_id: Option<String>,
}

impl MembersRemoveConsolidationRequestBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn member_company_id(mut self, value: impl Into<String>) -> Self {
        self.member_company_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MembersRemoveConsolidationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](MembersRemoveConsolidationRequestBuilder::group_id)
    /// - [`member_company_id`](MembersRemoveConsolidationRequestBuilder::member_company_id)
    pub fn build(self) -> Result<MembersRemoveConsolidationRequest, BuildError> {
        Ok(MembersRemoveConsolidationRequest {
            group_id: self
                .group_id
                .ok_or_else(|| BuildError::missing_field("group_id"))?,
            member_company_id: self
                .member_company_id
                .ok_or_else(|| BuildError::missing_field("member_company_id"))?,
        })
    }
}

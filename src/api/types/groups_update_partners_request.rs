pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GroupsUpdatePartnersRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl GroupsUpdatePartnersRequest {
    pub fn builder() -> GroupsUpdatePartnersRequestBuilder {
        <GroupsUpdatePartnersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GroupsUpdatePartnersRequestBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
}

impl GroupsUpdatePartnersRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GroupsUpdatePartnersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GroupsUpdatePartnersRequestBuilder::id)
    pub fn build(self) -> Result<GroupsUpdatePartnersRequest, BuildError> {
        Ok(GroupsUpdatePartnersRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code,
            name: self.name,
        })
    }
}

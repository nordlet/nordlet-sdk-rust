pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GroupsDeleteConsolidationResponse {
    #[serde(default)]
    pub ok: bool,
}

impl GroupsDeleteConsolidationResponse {
    pub fn builder() -> GroupsDeleteConsolidationResponseBuilder {
        <GroupsDeleteConsolidationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GroupsDeleteConsolidationResponseBuilder {
    ok: Option<bool>,
}

impl GroupsDeleteConsolidationResponseBuilder {
    pub fn ok(mut self, value: bool) -> Self {
        self.ok = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GroupsDeleteConsolidationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ok`](GroupsDeleteConsolidationResponseBuilder::ok)
    pub fn build(self) -> Result<GroupsDeleteConsolidationResponse, BuildError> {
        Ok(GroupsDeleteConsolidationResponse {
            ok: self.ok.ok_or_else(|| BuildError::missing_field("ok"))?,
        })
    }
}

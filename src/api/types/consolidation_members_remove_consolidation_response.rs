pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MembersRemoveConsolidationResponse {
    #[serde(default)]
    pub ok: bool,
}

impl MembersRemoveConsolidationResponse {
    pub fn builder() -> MembersRemoveConsolidationResponseBuilder {
        <MembersRemoveConsolidationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembersRemoveConsolidationResponseBuilder {
    ok: Option<bool>,
}

impl MembersRemoveConsolidationResponseBuilder {
    pub fn ok(mut self, value: bool) -> Self {
        self.ok = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MembersRemoveConsolidationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ok`](MembersRemoveConsolidationResponseBuilder::ok)
    pub fn build(self) -> Result<MembersRemoveConsolidationResponse, BuildError> {
        Ok(MembersRemoveConsolidationResponse {
            ok: self.ok.ok_or_else(|| BuildError::missing_field("ok"))?,
        })
    }
}

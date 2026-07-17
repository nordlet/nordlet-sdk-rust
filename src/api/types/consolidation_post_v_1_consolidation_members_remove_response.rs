pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationMembersRemoveResponse {
    #[serde(default)]
    pub ok: bool,
}

impl PostV1ConsolidationMembersRemoveResponse {
    pub fn builder() -> PostV1ConsolidationMembersRemoveResponseBuilder {
        <PostV1ConsolidationMembersRemoveResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationMembersRemoveResponseBuilder {
    ok: Option<bool>,
}

impl PostV1ConsolidationMembersRemoveResponseBuilder {
    pub fn ok(mut self, value: bool) -> Self {
        self.ok = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationMembersRemoveResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ok`](PostV1ConsolidationMembersRemoveResponseBuilder::ok)
    pub fn build(self) -> Result<PostV1ConsolidationMembersRemoveResponse, BuildError> {
        Ok(PostV1ConsolidationMembersRemoveResponse {
            ok: self.ok.ok_or_else(|| BuildError::missing_field("ok"))?,
        })
    }
}

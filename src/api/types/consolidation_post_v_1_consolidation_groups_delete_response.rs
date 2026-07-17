pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationGroupsDeleteResponse {
    #[serde(default)]
    pub ok: bool,
}

impl PostV1ConsolidationGroupsDeleteResponse {
    pub fn builder() -> PostV1ConsolidationGroupsDeleteResponseBuilder {
        <PostV1ConsolidationGroupsDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationGroupsDeleteResponseBuilder {
    ok: Option<bool>,
}

impl PostV1ConsolidationGroupsDeleteResponseBuilder {
    pub fn ok(mut self, value: bool) -> Self {
        self.ok = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationGroupsDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ok`](PostV1ConsolidationGroupsDeleteResponseBuilder::ok)
    pub fn build(self) -> Result<PostV1ConsolidationGroupsDeleteResponse, BuildError> {
        Ok(PostV1ConsolidationGroupsDeleteResponse {
            ok: self.ok.ok_or_else(|| BuildError::missing_field("ok"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationIntercompanyLinksRemoveResponse {
    #[serde(default)]
    pub ok: bool,
}

impl PostV1ConsolidationIntercompanyLinksRemoveResponse {
    pub fn builder() -> PostV1ConsolidationIntercompanyLinksRemoveResponseBuilder {
        <PostV1ConsolidationIntercompanyLinksRemoveResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationIntercompanyLinksRemoveResponseBuilder {
    ok: Option<bool>,
}

impl PostV1ConsolidationIntercompanyLinksRemoveResponseBuilder {
    pub fn ok(mut self, value: bool) -> Self {
        self.ok = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationIntercompanyLinksRemoveResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ok`](PostV1ConsolidationIntercompanyLinksRemoveResponseBuilder::ok)
    pub fn build(self) -> Result<PostV1ConsolidationIntercompanyLinksRemoveResponse, BuildError> {
        Ok(PostV1ConsolidationIntercompanyLinksRemoveResponse {
            ok: self.ok.ok_or_else(|| BuildError::missing_field("ok"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IntercompanyLinksRemoveConsolidationResponse {
    #[serde(default)]
    pub ok: bool,
}

impl IntercompanyLinksRemoveConsolidationResponse {
    pub fn builder() -> IntercompanyLinksRemoveConsolidationResponseBuilder {
        <IntercompanyLinksRemoveConsolidationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IntercompanyLinksRemoveConsolidationResponseBuilder {
    ok: Option<bool>,
}

impl IntercompanyLinksRemoveConsolidationResponseBuilder {
    pub fn ok(mut self, value: bool) -> Self {
        self.ok = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IntercompanyLinksRemoveConsolidationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ok`](IntercompanyLinksRemoveConsolidationResponseBuilder::ok)
    pub fn build(self) -> Result<IntercompanyLinksRemoveConsolidationResponse, BuildError> {
        Ok(IntercompanyLinksRemoveConsolidationResponse {
            ok: self.ok.ok_or_else(|| BuildError::missing_field("ok"))?,
        })
    }
}

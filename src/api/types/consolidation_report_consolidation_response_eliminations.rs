pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportConsolidationResponseEliminations {
    #[serde(default)]
    pub applied: Vec<ReportConsolidationResponseEliminationsAppliedItem>,
    #[serde(default)]
    pub balanced: bool,
    #[serde(default)]
    pub net: String,
}

impl ReportConsolidationResponseEliminations {
    pub fn builder() -> ReportConsolidationResponseEliminationsBuilder {
        <ReportConsolidationResponseEliminationsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportConsolidationResponseEliminationsBuilder {
    applied: Option<Vec<ReportConsolidationResponseEliminationsAppliedItem>>,
    balanced: Option<bool>,
    net: Option<String>,
}

impl ReportConsolidationResponseEliminationsBuilder {
    pub fn applied(
        mut self,
        value: Vec<ReportConsolidationResponseEliminationsAppliedItem>,
    ) -> Self {
        self.applied = Some(value);
        self
    }

    pub fn balanced(mut self, value: bool) -> Self {
        self.balanced = Some(value);
        self
    }

    pub fn net(mut self, value: impl Into<String>) -> Self {
        self.net = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReportConsolidationResponseEliminations`].
    /// This method will fail if any of the following fields are not set:
    /// - [`applied`](ReportConsolidationResponseEliminationsBuilder::applied)
    /// - [`balanced`](ReportConsolidationResponseEliminationsBuilder::balanced)
    /// - [`net`](ReportConsolidationResponseEliminationsBuilder::net)
    pub fn build(self) -> Result<ReportConsolidationResponseEliminations, BuildError> {
        Ok(ReportConsolidationResponseEliminations {
            applied: self
                .applied
                .ok_or_else(|| BuildError::missing_field("applied"))?,
            balanced: self
                .balanced
                .ok_or_else(|| BuildError::missing_field("balanced"))?,
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
        })
    }
}

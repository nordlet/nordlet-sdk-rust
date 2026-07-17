pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationReportResponseEliminations {
    #[serde(default)]
    pub applied: Vec<PostV1ConsolidationReportResponseEliminationsAppliedItem>,
    #[serde(default)]
    pub balanced: bool,
    #[serde(default)]
    pub net: String,
}

impl PostV1ConsolidationReportResponseEliminations {
    pub fn builder() -> PostV1ConsolidationReportResponseEliminationsBuilder {
        <PostV1ConsolidationReportResponseEliminationsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationReportResponseEliminationsBuilder {
    applied: Option<Vec<PostV1ConsolidationReportResponseEliminationsAppliedItem>>,
    balanced: Option<bool>,
    net: Option<String>,
}

impl PostV1ConsolidationReportResponseEliminationsBuilder {
    pub fn applied(
        mut self,
        value: Vec<PostV1ConsolidationReportResponseEliminationsAppliedItem>,
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

    /// Consumes the builder and constructs a [`PostV1ConsolidationReportResponseEliminations`].
    /// This method will fail if any of the following fields are not set:
    /// - [`applied`](PostV1ConsolidationReportResponseEliminationsBuilder::applied)
    /// - [`balanced`](PostV1ConsolidationReportResponseEliminationsBuilder::balanced)
    /// - [`net`](PostV1ConsolidationReportResponseEliminationsBuilder::net)
    pub fn build(self) -> Result<PostV1ConsolidationReportResponseEliminations, BuildError> {
        Ok(PostV1ConsolidationReportResponseEliminations {
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

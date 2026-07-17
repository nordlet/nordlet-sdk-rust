pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsCashFlowResponseFinancing {
    #[serde(default)]
    pub inflow: String,
    #[serde(default)]
    pub outflow: String,
    #[serde(default)]
    pub net: String,
    #[serde(default)]
    pub rows: Vec<PostV1ReportsCashFlowResponseFinancingRowsItem>,
}

impl PostV1ReportsCashFlowResponseFinancing {
    pub fn builder() -> PostV1ReportsCashFlowResponseFinancingBuilder {
        <PostV1ReportsCashFlowResponseFinancingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsCashFlowResponseFinancingBuilder {
    inflow: Option<String>,
    outflow: Option<String>,
    net: Option<String>,
    rows: Option<Vec<PostV1ReportsCashFlowResponseFinancingRowsItem>>,
}

impl PostV1ReportsCashFlowResponseFinancingBuilder {
    pub fn inflow(mut self, value: impl Into<String>) -> Self {
        self.inflow = Some(value.into());
        self
    }

    pub fn outflow(mut self, value: impl Into<String>) -> Self {
        self.outflow = Some(value.into());
        self
    }

    pub fn net(mut self, value: impl Into<String>) -> Self {
        self.net = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<PostV1ReportsCashFlowResponseFinancingRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsCashFlowResponseFinancing`].
    /// This method will fail if any of the following fields are not set:
    /// - [`inflow`](PostV1ReportsCashFlowResponseFinancingBuilder::inflow)
    /// - [`outflow`](PostV1ReportsCashFlowResponseFinancingBuilder::outflow)
    /// - [`net`](PostV1ReportsCashFlowResponseFinancingBuilder::net)
    /// - [`rows`](PostV1ReportsCashFlowResponseFinancingBuilder::rows)
    pub fn build(self) -> Result<PostV1ReportsCashFlowResponseFinancing, BuildError> {
        Ok(PostV1ReportsCashFlowResponseFinancing {
            inflow: self
                .inflow
                .ok_or_else(|| BuildError::missing_field("inflow"))?,
            outflow: self
                .outflow
                .ok_or_else(|| BuildError::missing_field("outflow"))?,
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}

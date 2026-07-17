pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsCashFlowResponseOperating {
    #[serde(default)]
    pub inflow: String,
    #[serde(default)]
    pub outflow: String,
    #[serde(default)]
    pub net: String,
    #[serde(default)]
    pub rows: Vec<PostV1ReportsCashFlowResponseOperatingRowsItem>,
}

impl PostV1ReportsCashFlowResponseOperating {
    pub fn builder() -> PostV1ReportsCashFlowResponseOperatingBuilder {
        <PostV1ReportsCashFlowResponseOperatingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsCashFlowResponseOperatingBuilder {
    inflow: Option<String>,
    outflow: Option<String>,
    net: Option<String>,
    rows: Option<Vec<PostV1ReportsCashFlowResponseOperatingRowsItem>>,
}

impl PostV1ReportsCashFlowResponseOperatingBuilder {
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

    pub fn rows(mut self, value: Vec<PostV1ReportsCashFlowResponseOperatingRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsCashFlowResponseOperating`].
    /// This method will fail if any of the following fields are not set:
    /// - [`inflow`](PostV1ReportsCashFlowResponseOperatingBuilder::inflow)
    /// - [`outflow`](PostV1ReportsCashFlowResponseOperatingBuilder::outflow)
    /// - [`net`](PostV1ReportsCashFlowResponseOperatingBuilder::net)
    /// - [`rows`](PostV1ReportsCashFlowResponseOperatingBuilder::rows)
    pub fn build(self) -> Result<PostV1ReportsCashFlowResponseOperating, BuildError> {
        Ok(PostV1ReportsCashFlowResponseOperating {
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

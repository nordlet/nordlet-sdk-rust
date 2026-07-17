pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsCashFlowResponseOperatingRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub inflow: String,
    #[serde(default)]
    pub outflow: String,
}

impl PostV1ReportsCashFlowResponseOperatingRowsItem {
    pub fn builder() -> PostV1ReportsCashFlowResponseOperatingRowsItemBuilder {
        <PostV1ReportsCashFlowResponseOperatingRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsCashFlowResponseOperatingRowsItemBuilder {
    code: Option<String>,
    name: Option<String>,
    inflow: Option<String>,
    outflow: Option<String>,
}

impl PostV1ReportsCashFlowResponseOperatingRowsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn inflow(mut self, value: impl Into<String>) -> Self {
        self.inflow = Some(value.into());
        self
    }

    pub fn outflow(mut self, value: impl Into<String>) -> Self {
        self.outflow = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsCashFlowResponseOperatingRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1ReportsCashFlowResponseOperatingRowsItemBuilder::code)
    /// - [`name`](PostV1ReportsCashFlowResponseOperatingRowsItemBuilder::name)
    /// - [`inflow`](PostV1ReportsCashFlowResponseOperatingRowsItemBuilder::inflow)
    /// - [`outflow`](PostV1ReportsCashFlowResponseOperatingRowsItemBuilder::outflow)
    pub fn build(self) -> Result<PostV1ReportsCashFlowResponseOperatingRowsItem, BuildError> {
        Ok(PostV1ReportsCashFlowResponseOperatingRowsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            inflow: self
                .inflow
                .ok_or_else(|| BuildError::missing_field("inflow"))?,
            outflow: self
                .outflow
                .ok_or_else(|| BuildError::missing_field("outflow"))?,
        })
    }
}

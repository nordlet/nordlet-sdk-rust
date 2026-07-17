pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsCostCenterActivityResponseCostCenter {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
}

impl PostV1ReportsCostCenterActivityResponseCostCenter {
    pub fn builder() -> PostV1ReportsCostCenterActivityResponseCostCenterBuilder {
        <PostV1ReportsCostCenterActivityResponseCostCenterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsCostCenterActivityResponseCostCenterBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
}

impl PostV1ReportsCostCenterActivityResponseCostCenterBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsCostCenterActivityResponseCostCenter`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ReportsCostCenterActivityResponseCostCenterBuilder::id)
    /// - [`code`](PostV1ReportsCostCenterActivityResponseCostCenterBuilder::code)
    /// - [`name`](PostV1ReportsCostCenterActivityResponseCostCenterBuilder::name)
    pub fn build(self) -> Result<PostV1ReportsCostCenterActivityResponseCostCenter, BuildError> {
        Ok(PostV1ReportsCostCenterActivityResponseCostCenter {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

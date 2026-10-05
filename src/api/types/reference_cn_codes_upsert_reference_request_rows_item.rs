pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CnCodesUpsertReferenceRequestRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "nameLt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_lt: Option<String>,
    #[serde(rename = "supplementaryUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplementary_unit: Option<String>,
}

impl CnCodesUpsertReferenceRequestRowsItem {
    pub fn builder() -> CnCodesUpsertReferenceRequestRowsItemBuilder {
        <CnCodesUpsertReferenceRequestRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CnCodesUpsertReferenceRequestRowsItemBuilder {
    code: Option<String>,
    name: Option<String>,
    name_lt: Option<String>,
    supplementary_unit: Option<String>,
}

impl CnCodesUpsertReferenceRequestRowsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn name_lt(mut self, value: impl Into<String>) -> Self {
        self.name_lt = Some(value.into());
        self
    }

    pub fn supplementary_unit(mut self, value: impl Into<String>) -> Self {
        self.supplementary_unit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CnCodesUpsertReferenceRequestRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](CnCodesUpsertReferenceRequestRowsItemBuilder::code)
    /// - [`name`](CnCodesUpsertReferenceRequestRowsItemBuilder::name)
    pub fn build(self) -> Result<CnCodesUpsertReferenceRequestRowsItem, BuildError> {
        Ok(CnCodesUpsertReferenceRequestRowsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            name_lt: self.name_lt,
            supplementary_unit: self.supplementary_unit,
        })
    }
}

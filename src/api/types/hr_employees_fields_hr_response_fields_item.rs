pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EmployeesFieldsHrResponseFieldsItem {
    #[serde(default)]
    pub key: String,
    pub kind: EmployeesFieldsHrResponseFieldsItemKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    #[serde(rename = "maxLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,
}

impl EmployeesFieldsHrResponseFieldsItem {
    pub fn builder() -> EmployeesFieldsHrResponseFieldsItemBuilder {
        <EmployeesFieldsHrResponseFieldsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmployeesFieldsHrResponseFieldsItemBuilder {
    key: Option<String>,
    kind: Option<EmployeesFieldsHrResponseFieldsItemKind>,
    options: Option<Vec<String>>,
    max_length: Option<i64>,
}

impl EmployeesFieldsHrResponseFieldsItemBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn kind(mut self, value: EmployeesFieldsHrResponseFieldsItemKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn options(mut self, value: Vec<String>) -> Self {
        self.options = Some(value);
        self
    }

    pub fn max_length(mut self, value: i64) -> Self {
        self.max_length = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EmployeesFieldsHrResponseFieldsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](EmployeesFieldsHrResponseFieldsItemBuilder::key)
    /// - [`kind`](EmployeesFieldsHrResponseFieldsItemBuilder::kind)
    pub fn build(self) -> Result<EmployeesFieldsHrResponseFieldsItem, BuildError> {
        Ok(EmployeesFieldsHrResponseFieldsItem {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            options: self.options,
            max_length: self.max_length,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1HrEmployeesFieldsResponseFieldsItem {
    #[serde(default)]
    pub key: String,
    pub kind: PostV1HrEmployeesFieldsResponseFieldsItemKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    #[serde(rename = "maxLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,
}

impl PostV1HrEmployeesFieldsResponseFieldsItem {
    pub fn builder() -> PostV1HrEmployeesFieldsResponseFieldsItemBuilder {
        <PostV1HrEmployeesFieldsResponseFieldsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrEmployeesFieldsResponseFieldsItemBuilder {
    key: Option<String>,
    kind: Option<PostV1HrEmployeesFieldsResponseFieldsItemKind>,
    options: Option<Vec<String>>,
    max_length: Option<i64>,
}

impl PostV1HrEmployeesFieldsResponseFieldsItemBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn kind(mut self, value: PostV1HrEmployeesFieldsResponseFieldsItemKind) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1HrEmployeesFieldsResponseFieldsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](PostV1HrEmployeesFieldsResponseFieldsItemBuilder::key)
    /// - [`kind`](PostV1HrEmployeesFieldsResponseFieldsItemBuilder::kind)
    pub fn build(self) -> Result<PostV1HrEmployeesFieldsResponseFieldsItem, BuildError> {
        Ok(PostV1HrEmployeesFieldsResponseFieldsItem {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            options: self.options,
            max_length: self.max_length,
        })
    }
}

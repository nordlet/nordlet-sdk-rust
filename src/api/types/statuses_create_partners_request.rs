pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StatusesCreatePartnersRequest {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i64>,
}

impl StatusesCreatePartnersRequest {
    pub fn builder() -> StatusesCreatePartnersRequestBuilder {
        <StatusesCreatePartnersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatusesCreatePartnersRequestBuilder {
    code: Option<String>,
    name: Option<String>,
    sort_order: Option<i64>,
}

impl StatusesCreatePartnersRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn sort_order(mut self, value: i64) -> Self {
        self.sort_order = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StatusesCreatePartnersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](StatusesCreatePartnersRequestBuilder::code)
    /// - [`name`](StatusesCreatePartnersRequestBuilder::name)
    pub fn build(self) -> Result<StatusesCreatePartnersRequest, BuildError> {
        Ok(StatusesCreatePartnersRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            sort_order: self.sort_order,
        })
    }
}

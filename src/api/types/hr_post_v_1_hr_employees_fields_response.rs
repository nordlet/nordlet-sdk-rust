pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrEmployeesFieldsResponse {
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub fields: Vec<PostV1HrEmployeesFieldsResponseFieldsItem>,
}

impl PostV1HrEmployeesFieldsResponse {
    pub fn builder() -> PostV1HrEmployeesFieldsResponseBuilder {
        <PostV1HrEmployeesFieldsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrEmployeesFieldsResponseBuilder {
    country: Option<String>,
    fields: Option<Vec<PostV1HrEmployeesFieldsResponseFieldsItem>>,
}

impl PostV1HrEmployeesFieldsResponseBuilder {
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn fields(mut self, value: Vec<PostV1HrEmployeesFieldsResponseFieldsItem>) -> Self {
        self.fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrEmployeesFieldsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country`](PostV1HrEmployeesFieldsResponseBuilder::country)
    /// - [`fields`](PostV1HrEmployeesFieldsResponseBuilder::fields)
    pub fn build(self) -> Result<PostV1HrEmployeesFieldsResponse, BuildError> {
        Ok(PostV1HrEmployeesFieldsResponse {
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            fields: self
                .fields
                .ok_or_else(|| BuildError::missing_field("fields"))?,
        })
    }
}

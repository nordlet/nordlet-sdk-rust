pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AgreementsTypesCreateRequest {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
}

impl PostV1AgreementsTypesCreateRequest {
    pub fn builder() -> PostV1AgreementsTypesCreateRequestBuilder {
        <PostV1AgreementsTypesCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AgreementsTypesCreateRequestBuilder {
    code: Option<String>,
    name: Option<String>,
}

impl PostV1AgreementsTypesCreateRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AgreementsTypesCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1AgreementsTypesCreateRequestBuilder::code)
    /// - [`name`](PostV1AgreementsTypesCreateRequestBuilder::name)
    pub fn build(self) -> Result<PostV1AgreementsTypesCreateRequest, BuildError> {
        Ok(PostV1AgreementsTypesCreateRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

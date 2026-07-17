pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersGroupsCreateRequest {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
}

impl PostV1PartnersGroupsCreateRequest {
    pub fn builder() -> PostV1PartnersGroupsCreateRequestBuilder {
        <PostV1PartnersGroupsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersGroupsCreateRequestBuilder {
    code: Option<String>,
    name: Option<String>,
}

impl PostV1PartnersGroupsCreateRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersGroupsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1PartnersGroupsCreateRequestBuilder::code)
    /// - [`name`](PostV1PartnersGroupsCreateRequestBuilder::name)
    pub fn build(self) -> Result<PostV1PartnersGroupsCreateRequest, BuildError> {
        Ok(PostV1PartnersGroupsCreateRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AgreementsTypesCreateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
}

impl PostV1AgreementsTypesCreateResponse {
    pub fn builder() -> PostV1AgreementsTypesCreateResponseBuilder {
        <PostV1AgreementsTypesCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AgreementsTypesCreateResponseBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
}

impl PostV1AgreementsTypesCreateResponseBuilder {
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

    /// Consumes the builder and constructs a [`PostV1AgreementsTypesCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AgreementsTypesCreateResponseBuilder::id)
    /// - [`code`](PostV1AgreementsTypesCreateResponseBuilder::code)
    /// - [`name`](PostV1AgreementsTypesCreateResponseBuilder::name)
    pub fn build(self) -> Result<PostV1AgreementsTypesCreateResponse, BuildError> {
        Ok(PostV1AgreementsTypesCreateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

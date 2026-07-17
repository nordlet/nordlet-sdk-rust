pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1HrPositionsCreateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations:
        Option<HashMap<String, Option<PostV1HrPositionsCreateResponseTranslationsValue>>>,
}

impl PostV1HrPositionsCreateResponse {
    pub fn builder() -> PostV1HrPositionsCreateResponseBuilder {
        <PostV1HrPositionsCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrPositionsCreateResponseBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    translations: Option<HashMap<String, Option<PostV1HrPositionsCreateResponseTranslationsValue>>>,
}

impl PostV1HrPositionsCreateResponseBuilder {
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

    pub fn translations(
        mut self,
        value: HashMap<String, Option<PostV1HrPositionsCreateResponseTranslationsValue>>,
    ) -> Self {
        self.translations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrPositionsCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1HrPositionsCreateResponseBuilder::id)
    /// - [`name`](PostV1HrPositionsCreateResponseBuilder::name)
    pub fn build(self) -> Result<PostV1HrPositionsCreateResponse, BuildError> {
        Ok(PostV1HrPositionsCreateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            translations: self.translations,
        })
    }
}

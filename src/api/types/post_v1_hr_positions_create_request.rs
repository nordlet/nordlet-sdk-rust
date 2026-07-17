pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1HrPositionsCreateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations: Option<HashMap<String, PostV1HrPositionsCreateRequestTranslationsValue>>,
}

impl PostV1HrPositionsCreateRequest {
    pub fn builder() -> PostV1HrPositionsCreateRequestBuilder {
        <PostV1HrPositionsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrPositionsCreateRequestBuilder {
    code: Option<String>,
    name: Option<String>,
    translations: Option<HashMap<String, PostV1HrPositionsCreateRequestTranslationsValue>>,
}

impl PostV1HrPositionsCreateRequestBuilder {
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
        value: HashMap<String, PostV1HrPositionsCreateRequestTranslationsValue>,
    ) -> Self {
        self.translations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrPositionsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1HrPositionsCreateRequestBuilder::name)
    pub fn build(self) -> Result<PostV1HrPositionsCreateRequest, BuildError> {
        Ok(PostV1HrPositionsCreateRequest {
            code: self.code,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            translations: self.translations,
        })
    }
}

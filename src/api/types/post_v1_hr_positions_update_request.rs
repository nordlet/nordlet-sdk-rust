pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1HrPositionsUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations:
        Option<HashMap<String, Option<PostV1HrPositionsUpdateRequestTranslationsValue>>>,
}

impl PostV1HrPositionsUpdateRequest {
    pub fn builder() -> PostV1HrPositionsUpdateRequestBuilder {
        <PostV1HrPositionsUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrPositionsUpdateRequestBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    translations: Option<HashMap<String, Option<PostV1HrPositionsUpdateRequestTranslationsValue>>>,
}

impl PostV1HrPositionsUpdateRequestBuilder {
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
        value: HashMap<String, Option<PostV1HrPositionsUpdateRequestTranslationsValue>>,
    ) -> Self {
        self.translations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrPositionsUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1HrPositionsUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1HrPositionsUpdateRequest, BuildError> {
        Ok(PostV1HrPositionsUpdateRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code,
            name: self.name,
            translations: self.translations,
        })
    }
}

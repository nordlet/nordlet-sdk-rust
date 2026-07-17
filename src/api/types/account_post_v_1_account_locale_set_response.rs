pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1AccountLocaleSetResponse {
    #[serde(default)]
    pub locale: String,
    pub scope: PostV1AccountLocaleSetResponseScope,
}

impl PostV1AccountLocaleSetResponse {
    pub fn builder() -> PostV1AccountLocaleSetResponseBuilder {
        <PostV1AccountLocaleSetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountLocaleSetResponseBuilder {
    locale: Option<String>,
    scope: Option<PostV1AccountLocaleSetResponseScope>,
}

impl PostV1AccountLocaleSetResponseBuilder {
    pub fn locale(mut self, value: impl Into<String>) -> Self {
        self.locale = Some(value.into());
        self
    }

    pub fn scope(mut self, value: PostV1AccountLocaleSetResponseScope) -> Self {
        self.scope = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountLocaleSetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`locale`](PostV1AccountLocaleSetResponseBuilder::locale)
    /// - [`scope`](PostV1AccountLocaleSetResponseBuilder::scope)
    pub fn build(self) -> Result<PostV1AccountLocaleSetResponse, BuildError> {
        Ok(PostV1AccountLocaleSetResponse {
            locale: self
                .locale
                .ok_or_else(|| BuildError::missing_field("locale"))?,
            scope: self
                .scope
                .ok_or_else(|| BuildError::missing_field("scope"))?,
        })
    }
}

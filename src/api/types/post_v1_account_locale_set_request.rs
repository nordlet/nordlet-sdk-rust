pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1AccountLocaleSetRequest {
    pub locale: PostV1AccountLocaleSetRequestLocale,
}

impl PostV1AccountLocaleSetRequest {
    pub fn builder() -> PostV1AccountLocaleSetRequestBuilder {
        <PostV1AccountLocaleSetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountLocaleSetRequestBuilder {
    locale: Option<PostV1AccountLocaleSetRequestLocale>,
}

impl PostV1AccountLocaleSetRequestBuilder {
    pub fn locale(mut self, value: PostV1AccountLocaleSetRequestLocale) -> Self {
        self.locale = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountLocaleSetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`locale`](PostV1AccountLocaleSetRequestBuilder::locale)
    pub fn build(self) -> Result<PostV1AccountLocaleSetRequest, BuildError> {
        Ok(PostV1AccountLocaleSetRequest {
            locale: self
                .locale
                .ok_or_else(|| BuildError::missing_field("locale"))?,
        })
    }
}

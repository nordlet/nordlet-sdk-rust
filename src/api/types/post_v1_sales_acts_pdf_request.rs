pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesActsPdfRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<PostV1SalesActsPdfRequestLocale>,
}

impl PostV1SalesActsPdfRequest {
    pub fn builder() -> PostV1SalesActsPdfRequestBuilder {
        <PostV1SalesActsPdfRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesActsPdfRequestBuilder {
    id: Option<String>,
    locale: Option<PostV1SalesActsPdfRequestLocale>,
}

impl PostV1SalesActsPdfRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn locale(mut self, value: PostV1SalesActsPdfRequestLocale) -> Self {
        self.locale = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesActsPdfRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1SalesActsPdfRequestBuilder::id)
    pub fn build(self) -> Result<PostV1SalesActsPdfRequest, BuildError> {
        Ok(PostV1SalesActsPdfRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            locale: self.locale,
        })
    }
}

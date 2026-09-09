pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CalendarListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(rename = "includeDone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_done: Option<bool>,
}

impl PostV1CalendarListRequest {
    pub fn builder() -> PostV1CalendarListRequestBuilder {
        <PostV1CalendarListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CalendarListRequestBuilder {
    from: Option<String>,
    to: Option<String>,
    include_done: Option<bool>,
}

impl PostV1CalendarListRequestBuilder {
    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());
        self
    }

    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.to = Some(value.into());
        self
    }

    pub fn include_done(mut self, value: bool) -> Self {
        self.include_done = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CalendarListRequest`].
    pub fn build(self) -> Result<PostV1CalendarListRequest, BuildError> {
        Ok(PostV1CalendarListRequest {
            from: self.from,
            to: self.to,
            include_done: self.include_done,
        })
    }
}

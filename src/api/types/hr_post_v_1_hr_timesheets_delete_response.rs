pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrTimesheetsDeleteResponse {
    #[serde(default)]
    pub id: String,
}

impl PostV1HrTimesheetsDeleteResponse {
    pub fn builder() -> PostV1HrTimesheetsDeleteResponseBuilder {
        <PostV1HrTimesheetsDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrTimesheetsDeleteResponseBuilder {
    id: Option<String>,
}

impl PostV1HrTimesheetsDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrTimesheetsDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1HrTimesheetsDeleteResponseBuilder::id)
    pub fn build(self) -> Result<PostV1HrTimesheetsDeleteResponse, BuildError> {
        Ok(PostV1HrTimesheetsDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

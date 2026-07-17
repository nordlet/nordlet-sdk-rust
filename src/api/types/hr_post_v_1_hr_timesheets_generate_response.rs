pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrTimesheetsGenerateResponse {
    #[serde(default)]
    pub generated: i64,
}

impl PostV1HrTimesheetsGenerateResponse {
    pub fn builder() -> PostV1HrTimesheetsGenerateResponseBuilder {
        <PostV1HrTimesheetsGenerateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrTimesheetsGenerateResponseBuilder {
    generated: Option<i64>,
}

impl PostV1HrTimesheetsGenerateResponseBuilder {
    pub fn generated(mut self, value: i64) -> Self {
        self.generated = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrTimesheetsGenerateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`generated`](PostV1HrTimesheetsGenerateResponseBuilder::generated)
    pub fn build(self) -> Result<PostV1HrTimesheetsGenerateResponse, BuildError> {
        Ok(PostV1HrTimesheetsGenerateResponse {
            generated: self
                .generated
                .ok_or_else(|| BuildError::missing_field("generated"))?,
        })
    }
}

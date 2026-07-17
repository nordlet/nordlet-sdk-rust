pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceCnCodesUpsertResponse {
    #[serde(default)]
    pub upserted: i64,
}

impl PostV1ReferenceCnCodesUpsertResponse {
    pub fn builder() -> PostV1ReferenceCnCodesUpsertResponseBuilder {
        <PostV1ReferenceCnCodesUpsertResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceCnCodesUpsertResponseBuilder {
    upserted: Option<i64>,
}

impl PostV1ReferenceCnCodesUpsertResponseBuilder {
    pub fn upserted(mut self, value: i64) -> Self {
        self.upserted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceCnCodesUpsertResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`upserted`](PostV1ReferenceCnCodesUpsertResponseBuilder::upserted)
    pub fn build(self) -> Result<PostV1ReferenceCnCodesUpsertResponse, BuildError> {
        Ok(PostV1ReferenceCnCodesUpsertResponse {
            upserted: self
                .upserted
                .ok_or_else(|| BuildError::missing_field("upserted"))?,
        })
    }
}

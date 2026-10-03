pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountReferralConvertRequest {
    #[serde(default)]
    pub points: i64,
}

impl PostV1AccountReferralConvertRequest {
    pub fn builder() -> PostV1AccountReferralConvertRequestBuilder {
        <PostV1AccountReferralConvertRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountReferralConvertRequestBuilder {
    points: Option<i64>,
}

impl PostV1AccountReferralConvertRequestBuilder {
    pub fn points(mut self, value: i64) -> Self {
        self.points = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountReferralConvertRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`points`](PostV1AccountReferralConvertRequestBuilder::points)
    pub fn build(self) -> Result<PostV1AccountReferralConvertRequest, BuildError> {
        Ok(PostV1AccountReferralConvertRequest {
            points: self
                .points
                .ok_or_else(|| BuildError::missing_field("points"))?,
        })
    }
}

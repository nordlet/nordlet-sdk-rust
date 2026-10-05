pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReferralConvertAccountRequest {
    #[serde(default)]
    pub points: i64,
}

impl ReferralConvertAccountRequest {
    pub fn builder() -> ReferralConvertAccountRequestBuilder {
        <ReferralConvertAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReferralConvertAccountRequestBuilder {
    points: Option<i64>,
}

impl ReferralConvertAccountRequestBuilder {
    pub fn points(mut self, value: i64) -> Self {
        self.points = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReferralConvertAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`points`](ReferralConvertAccountRequestBuilder::points)
    pub fn build(self) -> Result<ReferralConvertAccountRequest, BuildError> {
        Ok(ReferralConvertAccountRequest {
            points: self
                .points
                .ok_or_else(|| BuildError::missing_field("points"))?,
        })
    }
}

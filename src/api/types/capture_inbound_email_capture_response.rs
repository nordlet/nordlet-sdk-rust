pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InboundEmailCaptureResponse {
    #[serde(default)]
    pub accepted: i64,
    #[serde(default)]
    pub skipped: i64,
    #[serde(rename = "captureIds")]
    #[serde(default)]
    pub capture_ids: Vec<String>,
}

impl InboundEmailCaptureResponse {
    pub fn builder() -> InboundEmailCaptureResponseBuilder {
        <InboundEmailCaptureResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InboundEmailCaptureResponseBuilder {
    accepted: Option<i64>,
    skipped: Option<i64>,
    capture_ids: Option<Vec<String>>,
}

impl InboundEmailCaptureResponseBuilder {
    pub fn accepted(mut self, value: i64) -> Self {
        self.accepted = Some(value);
        self
    }

    pub fn skipped(mut self, value: i64) -> Self {
        self.skipped = Some(value);
        self
    }

    pub fn capture_ids(mut self, value: Vec<String>) -> Self {
        self.capture_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InboundEmailCaptureResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`accepted`](InboundEmailCaptureResponseBuilder::accepted)
    /// - [`skipped`](InboundEmailCaptureResponseBuilder::skipped)
    /// - [`capture_ids`](InboundEmailCaptureResponseBuilder::capture_ids)
    pub fn build(self) -> Result<InboundEmailCaptureResponse, BuildError> {
        Ok(InboundEmailCaptureResponse {
            accepted: self
                .accepted
                .ok_or_else(|| BuildError::missing_field("accepted"))?,
            skipped: self
                .skipped
                .ok_or_else(|| BuildError::missing_field("skipped"))?,
            capture_ids: self
                .capture_ids
                .ok_or_else(|| BuildError::missing_field("capture_ids"))?,
        })
    }
}

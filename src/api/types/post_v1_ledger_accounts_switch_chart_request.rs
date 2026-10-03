pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsSwitchChartRequest {}

impl PostV1LedgerAccountsSwitchChartRequest {
    pub fn builder() -> PostV1LedgerAccountsSwitchChartRequestBuilder {
        <PostV1LedgerAccountsSwitchChartRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsSwitchChartRequestBuilder {}

impl PostV1LedgerAccountsSwitchChartRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1LedgerAccountsSwitchChartRequest`].
    pub fn build(self) -> Result<PostV1LedgerAccountsSwitchChartRequest, BuildError> {
        Ok(PostV1LedgerAccountsSwitchChartRequest {})
    }
}

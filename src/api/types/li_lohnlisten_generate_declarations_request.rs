pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LiLohnlistenGenerateDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
}

impl LiLohnlistenGenerateDeclarationsRequest {
    pub fn builder() -> LiLohnlistenGenerateDeclarationsRequestBuilder {
        <LiLohnlistenGenerateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LiLohnlistenGenerateDeclarationsRequestBuilder {
    year: Option<i64>,
}

impl LiLohnlistenGenerateDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LiLohnlistenGenerateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](LiLohnlistenGenerateDeclarationsRequestBuilder::year)
    pub fn build(self) -> Result<LiLohnlistenGenerateDeclarationsRequest, BuildError> {
        Ok(LiLohnlistenGenerateDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LiLohndeklarationGenerateDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
}

impl LiLohndeklarationGenerateDeclarationsRequest {
    pub fn builder() -> LiLohndeklarationGenerateDeclarationsRequestBuilder {
        <LiLohndeklarationGenerateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LiLohndeklarationGenerateDeclarationsRequestBuilder {
    year: Option<i64>,
}

impl LiLohndeklarationGenerateDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LiLohndeklarationGenerateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](LiLohndeklarationGenerateDeclarationsRequestBuilder::year)
    pub fn build(self) -> Result<LiLohndeklarationGenerateDeclarationsRequest, BuildError> {
        Ok(LiLohndeklarationGenerateDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}

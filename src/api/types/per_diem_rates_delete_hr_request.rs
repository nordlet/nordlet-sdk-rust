pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PerDiemRatesDeleteHrRequest {
    #[serde(default)]
    pub id: String,
}

impl PerDiemRatesDeleteHrRequest {
    pub fn builder() -> PerDiemRatesDeleteHrRequestBuilder {
        <PerDiemRatesDeleteHrRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PerDiemRatesDeleteHrRequestBuilder {
    id: Option<String>,
}

impl PerDiemRatesDeleteHrRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PerDiemRatesDeleteHrRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PerDiemRatesDeleteHrRequestBuilder::id)
    pub fn build(self) -> Result<PerDiemRatesDeleteHrRequest, BuildError> {
        Ok(PerDiemRatesDeleteHrRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

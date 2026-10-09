pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PerDiemRatesDeleteHrResponse {
    #[serde(default)]
    pub id: String,
}

impl PerDiemRatesDeleteHrResponse {
    pub fn builder() -> PerDiemRatesDeleteHrResponseBuilder {
        <PerDiemRatesDeleteHrResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PerDiemRatesDeleteHrResponseBuilder {
    id: Option<String>,
}

impl PerDiemRatesDeleteHrResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PerDiemRatesDeleteHrResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PerDiemRatesDeleteHrResponseBuilder::id)
    pub fn build(self) -> Result<PerDiemRatesDeleteHrResponse, BuildError> {
        Ok(PerDiemRatesDeleteHrResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

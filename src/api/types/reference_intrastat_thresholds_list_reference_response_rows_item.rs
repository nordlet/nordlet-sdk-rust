pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IntrastatThresholdsListReferenceResponseRowsItem {
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "arrivalsReporting")]
    #[serde(default)]
    pub arrivals_reporting: String,
    #[serde(rename = "dispatchesReporting")]
    #[serde(default)]
    pub dispatches_reporting: String,
    #[serde(rename = "arrivalsStatistical")]
    #[serde(default)]
    pub arrivals_statistical: String,
    #[serde(rename = "dispatchesStatistical")]
    #[serde(default)]
    pub dispatches_statistical: String,
}

impl IntrastatThresholdsListReferenceResponseRowsItem {
    pub fn builder() -> IntrastatThresholdsListReferenceResponseRowsItemBuilder {
        <IntrastatThresholdsListReferenceResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IntrastatThresholdsListReferenceResponseRowsItemBuilder {
    year: Option<i64>,
    arrivals_reporting: Option<String>,
    dispatches_reporting: Option<String>,
    arrivals_statistical: Option<String>,
    dispatches_statistical: Option<String>,
}

impl IntrastatThresholdsListReferenceResponseRowsItemBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn arrivals_reporting(mut self, value: impl Into<String>) -> Self {
        self.arrivals_reporting = Some(value.into());
        self
    }

    pub fn dispatches_reporting(mut self, value: impl Into<String>) -> Self {
        self.dispatches_reporting = Some(value.into());
        self
    }

    pub fn arrivals_statistical(mut self, value: impl Into<String>) -> Self {
        self.arrivals_statistical = Some(value.into());
        self
    }

    pub fn dispatches_statistical(mut self, value: impl Into<String>) -> Self {
        self.dispatches_statistical = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IntrastatThresholdsListReferenceResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](IntrastatThresholdsListReferenceResponseRowsItemBuilder::year)
    /// - [`arrivals_reporting`](IntrastatThresholdsListReferenceResponseRowsItemBuilder::arrivals_reporting)
    /// - [`dispatches_reporting`](IntrastatThresholdsListReferenceResponseRowsItemBuilder::dispatches_reporting)
    /// - [`arrivals_statistical`](IntrastatThresholdsListReferenceResponseRowsItemBuilder::arrivals_statistical)
    /// - [`dispatches_statistical`](IntrastatThresholdsListReferenceResponseRowsItemBuilder::dispatches_statistical)
    pub fn build(self) -> Result<IntrastatThresholdsListReferenceResponseRowsItem, BuildError> {
        Ok(IntrastatThresholdsListReferenceResponseRowsItem {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            arrivals_reporting: self
                .arrivals_reporting
                .ok_or_else(|| BuildError::missing_field("arrivals_reporting"))?,
            dispatches_reporting: self
                .dispatches_reporting
                .ok_or_else(|| BuildError::missing_field("dispatches_reporting"))?,
            arrivals_statistical: self
                .arrivals_statistical
                .ok_or_else(|| BuildError::missing_field("arrivals_statistical"))?,
            dispatches_statistical: self
                .dispatches_statistical
                .ok_or_else(|| BuildError::missing_field("dispatches_statistical"))?,
        })
    }
}

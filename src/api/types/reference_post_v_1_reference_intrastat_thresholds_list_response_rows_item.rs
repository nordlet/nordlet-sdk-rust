pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceIntrastatThresholdsListResponseRowsItem {
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

impl PostV1ReferenceIntrastatThresholdsListResponseRowsItem {
    pub fn builder() -> PostV1ReferenceIntrastatThresholdsListResponseRowsItemBuilder {
        <PostV1ReferenceIntrastatThresholdsListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceIntrastatThresholdsListResponseRowsItemBuilder {
    year: Option<i64>,
    arrivals_reporting: Option<String>,
    dispatches_reporting: Option<String>,
    arrivals_statistical: Option<String>,
    dispatches_statistical: Option<String>,
}

impl PostV1ReferenceIntrastatThresholdsListResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1ReferenceIntrastatThresholdsListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1ReferenceIntrastatThresholdsListResponseRowsItemBuilder::year)
    /// - [`arrivals_reporting`](PostV1ReferenceIntrastatThresholdsListResponseRowsItemBuilder::arrivals_reporting)
    /// - [`dispatches_reporting`](PostV1ReferenceIntrastatThresholdsListResponseRowsItemBuilder::dispatches_reporting)
    /// - [`arrivals_statistical`](PostV1ReferenceIntrastatThresholdsListResponseRowsItemBuilder::arrivals_statistical)
    /// - [`dispatches_statistical`](PostV1ReferenceIntrastatThresholdsListResponseRowsItemBuilder::dispatches_statistical)
    pub fn build(
        self,
    ) -> Result<PostV1ReferenceIntrastatThresholdsListResponseRowsItem, BuildError> {
        Ok(PostV1ReferenceIntrastatThresholdsListResponseRowsItem {
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

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlJpkKrGenerateDeclarationsRequest {
    #[serde(rename = "dateFrom")]
    #[serde(default)]
    pub date_from: NaiveDate,
    #[serde(rename = "dateTo")]
    #[serde(default)]
    pub date_to: NaiveDate,
}

impl PlJpkKrGenerateDeclarationsRequest {
    pub fn builder() -> PlJpkKrGenerateDeclarationsRequestBuilder {
        <PlJpkKrGenerateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlJpkKrGenerateDeclarationsRequestBuilder {
    date_from: Option<NaiveDate>,
    date_to: Option<NaiveDate>,
}

impl PlJpkKrGenerateDeclarationsRequestBuilder {
    pub fn date_from(mut self, value: NaiveDate) -> Self {
        self.date_from = Some(value);
        self
    }

    pub fn date_to(mut self, value: NaiveDate) -> Self {
        self.date_to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlJpkKrGenerateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date_from`](PlJpkKrGenerateDeclarationsRequestBuilder::date_from)
    /// - [`date_to`](PlJpkKrGenerateDeclarationsRequestBuilder::date_to)
    pub fn build(self) -> Result<PlJpkKrGenerateDeclarationsRequest, BuildError> {
        Ok(PlJpkKrGenerateDeclarationsRequest {
            date_from: self
                .date_from
                .ok_or_else(|| BuildError::missing_field("date_from"))?,
            date_to: self
                .date_to
                .ok_or_else(|| BuildError::missing_field("date_to"))?,
        })
    }
}

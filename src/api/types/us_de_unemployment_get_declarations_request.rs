pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsDeUnemploymentGetDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub quarter: i64,
}

impl UsDeUnemploymentGetDeclarationsRequest {
    pub fn builder() -> UsDeUnemploymentGetDeclarationsRequestBuilder {
        <UsDeUnemploymentGetDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsDeUnemploymentGetDeclarationsRequestBuilder {
    year: Option<i64>,
    quarter: Option<i64>,
}

impl UsDeUnemploymentGetDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn quarter(mut self, value: i64) -> Self {
        self.quarter = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsDeUnemploymentGetDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](UsDeUnemploymentGetDeclarationsRequestBuilder::year)
    /// - [`quarter`](UsDeUnemploymentGetDeclarationsRequestBuilder::quarter)
    pub fn build(self) -> Result<UsDeUnemploymentGetDeclarationsRequest, BuildError> {
        Ok(UsDeUnemploymentGetDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            quarter: self
                .quarter
                .ok_or_else(|| BuildError::missing_field("quarter"))?,
        })
    }
}

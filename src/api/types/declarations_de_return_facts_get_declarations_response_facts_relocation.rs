pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeReturnFactsGetDeclarationsResponseFactsRelocation {
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(default)]
    pub from: String,
    #[serde(default)]
    pub to: String,
}

impl DeReturnFactsGetDeclarationsResponseFactsRelocation {
    pub fn builder() -> DeReturnFactsGetDeclarationsResponseFactsRelocationBuilder {
        <DeReturnFactsGetDeclarationsResponseFactsRelocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeReturnFactsGetDeclarationsResponseFactsRelocationBuilder {
    date: Option<NaiveDate>,
    from: Option<String>,
    to: Option<String>,
}

impl DeReturnFactsGetDeclarationsResponseFactsRelocationBuilder {
    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());
        self
    }

    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.to = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeReturnFactsGetDeclarationsResponseFactsRelocation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](DeReturnFactsGetDeclarationsResponseFactsRelocationBuilder::date)
    /// - [`from`](DeReturnFactsGetDeclarationsResponseFactsRelocationBuilder::from)
    /// - [`to`](DeReturnFactsGetDeclarationsResponseFactsRelocationBuilder::to)
    pub fn build(self) -> Result<DeReturnFactsGetDeclarationsResponseFactsRelocation, BuildError> {
        Ok(DeReturnFactsGetDeclarationsResponseFactsRelocation {
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeReturnFactsSetDeclarationsRequestFactsRelocation {
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(default)]
    pub from: String,
    #[serde(default)]
    pub to: String,
}

impl DeReturnFactsSetDeclarationsRequestFactsRelocation {
    pub fn builder() -> DeReturnFactsSetDeclarationsRequestFactsRelocationBuilder {
        <DeReturnFactsSetDeclarationsRequestFactsRelocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeReturnFactsSetDeclarationsRequestFactsRelocationBuilder {
    date: Option<NaiveDate>,
    from: Option<String>,
    to: Option<String>,
}

impl DeReturnFactsSetDeclarationsRequestFactsRelocationBuilder {
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

    /// Consumes the builder and constructs a [`DeReturnFactsSetDeclarationsRequestFactsRelocation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](DeReturnFactsSetDeclarationsRequestFactsRelocationBuilder::date)
    /// - [`from`](DeReturnFactsSetDeclarationsRequestFactsRelocationBuilder::from)
    /// - [`to`](DeReturnFactsSetDeclarationsRequestFactsRelocationBuilder::to)
    pub fn build(self) -> Result<DeReturnFactsSetDeclarationsRequestFactsRelocation, BuildError> {
        Ok(DeReturnFactsSetDeclarationsRequestFactsRelocation {
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
        })
    }
}

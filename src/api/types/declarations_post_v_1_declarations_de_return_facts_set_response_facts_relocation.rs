pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsDeReturnFactsSetResponseFactsRelocation {
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub from: String,
    #[serde(default)]
    pub to: String,
}

impl PostV1DeclarationsDeReturnFactsSetResponseFactsRelocation {
    pub fn builder() -> PostV1DeclarationsDeReturnFactsSetResponseFactsRelocationBuilder {
        <PostV1DeclarationsDeReturnFactsSetResponseFactsRelocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsDeReturnFactsSetResponseFactsRelocationBuilder {
    date: Option<String>,
    from: Option<String>,
    to: Option<String>,
}

impl PostV1DeclarationsDeReturnFactsSetResponseFactsRelocationBuilder {
    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsDeReturnFactsSetResponseFactsRelocation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](PostV1DeclarationsDeReturnFactsSetResponseFactsRelocationBuilder::date)
    /// - [`from`](PostV1DeclarationsDeReturnFactsSetResponseFactsRelocationBuilder::from)
    /// - [`to`](PostV1DeclarationsDeReturnFactsSetResponseFactsRelocationBuilder::to)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsDeReturnFactsSetResponseFactsRelocation, BuildError> {
        Ok(PostV1DeclarationsDeReturnFactsSetResponseFactsRelocation {
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
        })
    }
}

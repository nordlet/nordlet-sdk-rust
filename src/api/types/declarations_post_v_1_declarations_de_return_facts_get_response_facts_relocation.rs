pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsDeReturnFactsGetResponseFactsRelocation {
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub from: String,
    #[serde(default)]
    pub to: String,
}

impl PostV1DeclarationsDeReturnFactsGetResponseFactsRelocation {
    pub fn builder() -> PostV1DeclarationsDeReturnFactsGetResponseFactsRelocationBuilder {
        <PostV1DeclarationsDeReturnFactsGetResponseFactsRelocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsDeReturnFactsGetResponseFactsRelocationBuilder {
    date: Option<String>,
    from: Option<String>,
    to: Option<String>,
}

impl PostV1DeclarationsDeReturnFactsGetResponseFactsRelocationBuilder {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsDeReturnFactsGetResponseFactsRelocation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](PostV1DeclarationsDeReturnFactsGetResponseFactsRelocationBuilder::date)
    /// - [`from`](PostV1DeclarationsDeReturnFactsGetResponseFactsRelocationBuilder::from)
    /// - [`to`](PostV1DeclarationsDeReturnFactsGetResponseFactsRelocationBuilder::to)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsDeReturnFactsGetResponseFactsRelocation, BuildError> {
        Ok(PostV1DeclarationsDeReturnFactsGetResponseFactsRelocation {
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
        })
    }
}

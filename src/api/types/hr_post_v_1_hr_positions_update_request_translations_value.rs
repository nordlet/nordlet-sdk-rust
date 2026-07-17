pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrPositionsUpdateRequestTranslationsValue {
    #[serde(default)]
    pub name: String,
}

impl PostV1HrPositionsUpdateRequestTranslationsValue {
    pub fn builder() -> PostV1HrPositionsUpdateRequestTranslationsValueBuilder {
        <PostV1HrPositionsUpdateRequestTranslationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrPositionsUpdateRequestTranslationsValueBuilder {
    name: Option<String>,
}

impl PostV1HrPositionsUpdateRequestTranslationsValueBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrPositionsUpdateRequestTranslationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1HrPositionsUpdateRequestTranslationsValueBuilder::name)
    pub fn build(self) -> Result<PostV1HrPositionsUpdateRequestTranslationsValue, BuildError> {
        Ok(PostV1HrPositionsUpdateRequestTranslationsValue {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

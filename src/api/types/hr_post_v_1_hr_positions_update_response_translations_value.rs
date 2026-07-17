pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrPositionsUpdateResponseTranslationsValue {
    #[serde(default)]
    pub name: String,
}

impl PostV1HrPositionsUpdateResponseTranslationsValue {
    pub fn builder() -> PostV1HrPositionsUpdateResponseTranslationsValueBuilder {
        <PostV1HrPositionsUpdateResponseTranslationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrPositionsUpdateResponseTranslationsValueBuilder {
    name: Option<String>,
}

impl PostV1HrPositionsUpdateResponseTranslationsValueBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrPositionsUpdateResponseTranslationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1HrPositionsUpdateResponseTranslationsValueBuilder::name)
    pub fn build(self) -> Result<PostV1HrPositionsUpdateResponseTranslationsValue, BuildError> {
        Ok(PostV1HrPositionsUpdateResponseTranslationsValue {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

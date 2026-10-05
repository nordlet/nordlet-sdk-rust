pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PositionsUpdateHrRequestTranslationsValue {
    #[serde(default)]
    pub name: String,
}

impl PositionsUpdateHrRequestTranslationsValue {
    pub fn builder() -> PositionsUpdateHrRequestTranslationsValueBuilder {
        <PositionsUpdateHrRequestTranslationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PositionsUpdateHrRequestTranslationsValueBuilder {
    name: Option<String>,
}

impl PositionsUpdateHrRequestTranslationsValueBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PositionsUpdateHrRequestTranslationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PositionsUpdateHrRequestTranslationsValueBuilder::name)
    pub fn build(self) -> Result<PositionsUpdateHrRequestTranslationsValue, BuildError> {
        Ok(PositionsUpdateHrRequestTranslationsValue {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

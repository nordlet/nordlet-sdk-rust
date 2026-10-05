pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IeB1GenerateDeclarationsResponseMembersItem {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "sharesQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares_quantity: Option<String>,
    #[serde(rename = "sharesAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares_amount: Option<String>,
    #[serde(rename = "sharesType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares_type: Option<String>,
    #[serde(rename = "acquisitionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquisition_date: Option<NaiveDate>,
}

impl IeB1GenerateDeclarationsResponseMembersItem {
    pub fn builder() -> IeB1GenerateDeclarationsResponseMembersItemBuilder {
        <IeB1GenerateDeclarationsResponseMembersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IeB1GenerateDeclarationsResponseMembersItemBuilder {
    name: Option<String>,
    identifier: Option<String>,
    shares_quantity: Option<String>,
    shares_amount: Option<String>,
    shares_type: Option<String>,
    acquisition_date: Option<NaiveDate>,
}

impl IeB1GenerateDeclarationsResponseMembersItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn identifier(mut self, value: impl Into<String>) -> Self {
        self.identifier = Some(value.into());
        self
    }

    pub fn shares_quantity(mut self, value: impl Into<String>) -> Self {
        self.shares_quantity = Some(value.into());
        self
    }

    pub fn shares_amount(mut self, value: impl Into<String>) -> Self {
        self.shares_amount = Some(value.into());
        self
    }

    pub fn shares_type(mut self, value: impl Into<String>) -> Self {
        self.shares_type = Some(value.into());
        self
    }

    pub fn acquisition_date(mut self, value: NaiveDate) -> Self {
        self.acquisition_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IeB1GenerateDeclarationsResponseMembersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](IeB1GenerateDeclarationsResponseMembersItemBuilder::name)
    pub fn build(self) -> Result<IeB1GenerateDeclarationsResponseMembersItem, BuildError> {
        Ok(IeB1GenerateDeclarationsResponseMembersItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            identifier: self.identifier,
            shares_quantity: self.shares_quantity,
            shares_amount: self.shares_amount,
            shares_type: self.shares_type,
            acquisition_date: self.acquisition_date,
        })
    }
}

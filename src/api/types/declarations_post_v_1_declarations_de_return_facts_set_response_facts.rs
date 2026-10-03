pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsDeReturnFactsSetResponseFacts {
    #[serde(rename = "changedShareholderIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changed_shareholder_ids: Option<Vec<String>>,
    #[serde(rename = "shareholderContracts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shareholder_contracts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracts: Option<Vec<PostV1DeclarationsDeReturnFactsSetResponseFactsContractsItem>>,
    #[serde(rename = "harmfulShareAcquisition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harmful_share_acquisition: Option<bool>,
    #[serde(rename = "coronaAid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corona_aid: Option<String>,
    #[serde(rename = "lossCarryback")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loss_carryback: Option<String>,
    #[serde(rename = "donationCarryforward")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub donation_carryforward: Option<String>,
    #[serde(rename = "contributionAccountOpening")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contribution_account_opening: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributions:
        Option<Vec<PostV1DeclarationsDeReturnFactsSetResponseFactsContributionsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distributions:
        Option<Vec<PostV1DeclarationsDeReturnFactsSetResponseFactsDistributionsItem>>,
    #[serde(rename = "taxBalanceEquity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_balance_equity: Option<String>,
    #[serde(rename = "multipleMunicipalities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_municipalities: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relocation: Option<PostV1DeclarationsDeReturnFactsSetResponseFactsRelocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub municipalities:
        Option<Vec<PostV1DeclarationsDeReturnFactsSetResponseFactsMunicipalitiesItem>>,
    #[serde(rename = "landHoldings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub land_holdings: Option<Vec<PostV1DeclarationsDeReturnFactsSetResponseFactsLandHoldingsItem>>,
    #[serde(rename = "propertyTaxExpense")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_tax_expense: Option<String>,
    #[serde(rename = "licencesToNonResidents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub licences_to_non_residents: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participations:
        Option<Vec<PostV1DeclarationsDeReturnFactsSetResponseFactsParticipationsItem>>,
    #[serde(rename = "foreignIncome")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreign_income:
        Option<Vec<PostV1DeclarationsDeReturnFactsSetResponseFactsForeignIncomeItem>>,
    #[serde(rename = "smallBusinessSwitchDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_business_switch_date: Option<String>,
    #[serde(rename = "refundProcedureApplied")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_procedure_applied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub representative: Option<PostV1DeclarationsDeReturnFactsSetResponseFactsRepresentative>,
    #[serde(rename = "singleTransportTax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_transport_tax: Option<String>,
    #[serde(rename = "distanceSales")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_sales: Option<String>,
}

impl PostV1DeclarationsDeReturnFactsSetResponseFacts {
    pub fn builder() -> PostV1DeclarationsDeReturnFactsSetResponseFactsBuilder {
        <PostV1DeclarationsDeReturnFactsSetResponseFactsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsDeReturnFactsSetResponseFactsBuilder {
    changed_shareholder_ids: Option<Vec<String>>,
    shareholder_contracts: Option<bool>,
    contracts: Option<Vec<PostV1DeclarationsDeReturnFactsSetResponseFactsContractsItem>>,
    harmful_share_acquisition: Option<bool>,
    corona_aid: Option<String>,
    loss_carryback: Option<String>,
    donation_carryforward: Option<String>,
    contribution_account_opening: Option<String>,
    contributions: Option<Vec<PostV1DeclarationsDeReturnFactsSetResponseFactsContributionsItem>>,
    distributions: Option<Vec<PostV1DeclarationsDeReturnFactsSetResponseFactsDistributionsItem>>,
    tax_balance_equity: Option<String>,
    multiple_municipalities: Option<bool>,
    relocation: Option<PostV1DeclarationsDeReturnFactsSetResponseFactsRelocation>,
    municipalities: Option<Vec<PostV1DeclarationsDeReturnFactsSetResponseFactsMunicipalitiesItem>>,
    land_holdings: Option<Vec<PostV1DeclarationsDeReturnFactsSetResponseFactsLandHoldingsItem>>,
    property_tax_expense: Option<String>,
    licences_to_non_residents: Option<String>,
    participations: Option<Vec<PostV1DeclarationsDeReturnFactsSetResponseFactsParticipationsItem>>,
    foreign_income: Option<Vec<PostV1DeclarationsDeReturnFactsSetResponseFactsForeignIncomeItem>>,
    small_business_switch_date: Option<String>,
    refund_procedure_applied: Option<bool>,
    bic: Option<String>,
    representative: Option<PostV1DeclarationsDeReturnFactsSetResponseFactsRepresentative>,
    single_transport_tax: Option<String>,
    distance_sales: Option<String>,
}

impl PostV1DeclarationsDeReturnFactsSetResponseFactsBuilder {
    pub fn changed_shareholder_ids(mut self, value: Vec<String>) -> Self {
        self.changed_shareholder_ids = Some(value);
        self
    }

    pub fn shareholder_contracts(mut self, value: bool) -> Self {
        self.shareholder_contracts = Some(value);
        self
    }

    pub fn contracts(
        mut self,
        value: Vec<PostV1DeclarationsDeReturnFactsSetResponseFactsContractsItem>,
    ) -> Self {
        self.contracts = Some(value);
        self
    }

    pub fn harmful_share_acquisition(mut self, value: bool) -> Self {
        self.harmful_share_acquisition = Some(value);
        self
    }

    pub fn corona_aid(mut self, value: impl Into<String>) -> Self {
        self.corona_aid = Some(value.into());
        self
    }

    pub fn loss_carryback(mut self, value: impl Into<String>) -> Self {
        self.loss_carryback = Some(value.into());
        self
    }

    pub fn donation_carryforward(mut self, value: impl Into<String>) -> Self {
        self.donation_carryforward = Some(value.into());
        self
    }

    pub fn contribution_account_opening(mut self, value: impl Into<String>) -> Self {
        self.contribution_account_opening = Some(value.into());
        self
    }

    pub fn contributions(
        mut self,
        value: Vec<PostV1DeclarationsDeReturnFactsSetResponseFactsContributionsItem>,
    ) -> Self {
        self.contributions = Some(value);
        self
    }

    pub fn distributions(
        mut self,
        value: Vec<PostV1DeclarationsDeReturnFactsSetResponseFactsDistributionsItem>,
    ) -> Self {
        self.distributions = Some(value);
        self
    }

    pub fn tax_balance_equity(mut self, value: impl Into<String>) -> Self {
        self.tax_balance_equity = Some(value.into());
        self
    }

    pub fn multiple_municipalities(mut self, value: bool) -> Self {
        self.multiple_municipalities = Some(value);
        self
    }

    pub fn relocation(
        mut self,
        value: PostV1DeclarationsDeReturnFactsSetResponseFactsRelocation,
    ) -> Self {
        self.relocation = Some(value);
        self
    }

    pub fn municipalities(
        mut self,
        value: Vec<PostV1DeclarationsDeReturnFactsSetResponseFactsMunicipalitiesItem>,
    ) -> Self {
        self.municipalities = Some(value);
        self
    }

    pub fn land_holdings(
        mut self,
        value: Vec<PostV1DeclarationsDeReturnFactsSetResponseFactsLandHoldingsItem>,
    ) -> Self {
        self.land_holdings = Some(value);
        self
    }

    pub fn property_tax_expense(mut self, value: impl Into<String>) -> Self {
        self.property_tax_expense = Some(value.into());
        self
    }

    pub fn licences_to_non_residents(mut self, value: impl Into<String>) -> Self {
        self.licences_to_non_residents = Some(value.into());
        self
    }

    pub fn participations(
        mut self,
        value: Vec<PostV1DeclarationsDeReturnFactsSetResponseFactsParticipationsItem>,
    ) -> Self {
        self.participations = Some(value);
        self
    }

    pub fn foreign_income(
        mut self,
        value: Vec<PostV1DeclarationsDeReturnFactsSetResponseFactsForeignIncomeItem>,
    ) -> Self {
        self.foreign_income = Some(value);
        self
    }

    pub fn small_business_switch_date(mut self, value: impl Into<String>) -> Self {
        self.small_business_switch_date = Some(value.into());
        self
    }

    pub fn refund_procedure_applied(mut self, value: bool) -> Self {
        self.refund_procedure_applied = Some(value);
        self
    }

    pub fn bic(mut self, value: impl Into<String>) -> Self {
        self.bic = Some(value.into());
        self
    }

    pub fn representative(
        mut self,
        value: PostV1DeclarationsDeReturnFactsSetResponseFactsRepresentative,
    ) -> Self {
        self.representative = Some(value);
        self
    }

    pub fn single_transport_tax(mut self, value: impl Into<String>) -> Self {
        self.single_transport_tax = Some(value.into());
        self
    }

    pub fn distance_sales(mut self, value: impl Into<String>) -> Self {
        self.distance_sales = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsDeReturnFactsSetResponseFacts`].
    pub fn build(self) -> Result<PostV1DeclarationsDeReturnFactsSetResponseFacts, BuildError> {
        Ok(PostV1DeclarationsDeReturnFactsSetResponseFacts {
            changed_shareholder_ids: self.changed_shareholder_ids,
            shareholder_contracts: self.shareholder_contracts,
            contracts: self.contracts,
            harmful_share_acquisition: self.harmful_share_acquisition,
            corona_aid: self.corona_aid,
            loss_carryback: self.loss_carryback,
            donation_carryforward: self.donation_carryforward,
            contribution_account_opening: self.contribution_account_opening,
            contributions: self.contributions,
            distributions: self.distributions,
            tax_balance_equity: self.tax_balance_equity,
            multiple_municipalities: self.multiple_municipalities,
            relocation: self.relocation,
            municipalities: self.municipalities,
            land_holdings: self.land_holdings,
            property_tax_expense: self.property_tax_expense,
            licences_to_non_residents: self.licences_to_non_residents,
            participations: self.participations,
            foreign_income: self.foreign_income,
            small_business_switch_date: self.small_business_switch_date,
            refund_procedure_applied: self.refund_procedure_applied,
            bic: self.bic,
            representative: self.representative,
            single_transport_tax: self.single_transport_tax,
            distance_sales: self.distance_sales,
        })
    }
}

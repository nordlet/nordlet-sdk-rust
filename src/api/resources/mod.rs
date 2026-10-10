//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **reference**
//! - **partners**
//! - **Leads**
//! - **catalog**
//! - **sales**
//! - **OperationTypes**
//! - **DocumentSeries**
//! - **purchases**
//! - **capture**
//! - **peppol**
//! - **declarations**
//! - **ledger**
//! - **Officers**
//! - **PlatformSellers**
//! - **migration**
//! - **assets**
//! - **hr**
//! - **fleet**
//! - **payroll**
//! - **agreements**
//! - **inventory**
//! - **production**
//! - **ecommerce**
//! - **cash**
//! - **projects**
//! - **transport**
//! - **pos**
//! - **calendar**
//! - **audit**
//! - **webhooks**
//! - **bank**
//! - **files**
//! - **reports**
//! - **consolidation**
//! - **public**
//! - **billing**
//! - **account**

use crate::{ApiError, ClientConfig};

pub mod account;
pub mod agreements;
pub mod assets;
pub mod audit;
pub mod bank;
pub mod billing;
pub mod calendar;
pub mod capture;
pub mod cash;
pub mod catalog;
pub mod consolidation;
pub mod declarations;
pub mod document_series;
pub mod ecommerce;
pub mod files;
pub mod fleet;
pub mod hr;
pub mod inventory;
pub mod leads;
pub mod ledger;
pub mod migration;
pub mod officers;
pub mod operation_types;
pub mod partners;
pub mod payroll;
pub mod peppol;
pub mod platform_sellers;
pub mod pos;
pub mod production;
pub mod projects;
pub mod public;
pub mod purchases;
pub mod reference;
pub mod reports;
pub mod sales;
pub mod transport;
pub mod webhooks;
pub struct ApiClient {
    pub config: ClientConfig,
    pub reference: ReferenceClient,
    pub partners: PartnersClient,
    pub leads: LeadsClient,
    pub catalog: CatalogClient,
    pub sales: SalesClient,
    pub operation_types: OperationTypesClient,
    pub document_series: DocumentSeriesClient,
    pub purchases: PurchasesClient,
    pub capture: CaptureClient,
    pub peppol: PeppolClient,
    pub declarations: DeclarationsClient,
    pub ledger: LedgerClient,
    pub officers: OfficersClient,
    pub platform_sellers: PlatformSellersClient,
    pub migration: MigrationClient,
    pub assets: AssetsClient,
    pub hr: HrClient,
    pub fleet: FleetClient,
    pub payroll: PayrollClient,
    pub agreements: AgreementsClient,
    pub inventory: InventoryClient,
    pub production: ProductionClient,
    pub ecommerce: EcommerceClient,
    pub cash: CashClient,
    pub projects: ProjectsClient,
    pub transport: TransportClient,
    pub pos: PosClient,
    pub calendar: CalendarClient,
    pub audit: AuditClient,
    pub webhooks: WebhooksClient,
    pub bank: BankClient,
    pub files: FilesClient,
    pub reports: ReportsClient,
    pub consolidation: ConsolidationClient,
    pub public: PublicClient,
    pub billing: BillingClient,
    pub account: AccountClient,
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            reference: ReferenceClient::new(config.clone())?,
            partners: PartnersClient::new(config.clone())?,
            leads: LeadsClient::new(config.clone())?,
            catalog: CatalogClient::new(config.clone())?,
            sales: SalesClient::new(config.clone())?,
            operation_types: OperationTypesClient::new(config.clone())?,
            document_series: DocumentSeriesClient::new(config.clone())?,
            purchases: PurchasesClient::new(config.clone())?,
            capture: CaptureClient::new(config.clone())?,
            peppol: PeppolClient::new(config.clone())?,
            declarations: DeclarationsClient::new(config.clone())?,
            ledger: LedgerClient::new(config.clone())?,
            officers: OfficersClient::new(config.clone())?,
            platform_sellers: PlatformSellersClient::new(config.clone())?,
            migration: MigrationClient::new(config.clone())?,
            assets: AssetsClient::new(config.clone())?,
            hr: HrClient::new(config.clone())?,
            fleet: FleetClient::new(config.clone())?,
            payroll: PayrollClient::new(config.clone())?,
            agreements: AgreementsClient::new(config.clone())?,
            inventory: InventoryClient::new(config.clone())?,
            production: ProductionClient::new(config.clone())?,
            ecommerce: EcommerceClient::new(config.clone())?,
            cash: CashClient::new(config.clone())?,
            projects: ProjectsClient::new(config.clone())?,
            transport: TransportClient::new(config.clone())?,
            pos: PosClient::new(config.clone())?,
            calendar: CalendarClient::new(config.clone())?,
            audit: AuditClient::new(config.clone())?,
            webhooks: WebhooksClient::new(config.clone())?,
            bank: BankClient::new(config.clone())?,
            files: FilesClient::new(config.clone())?,
            reports: ReportsClient::new(config.clone())?,
            consolidation: ConsolidationClient::new(config.clone())?,
            public: PublicClient::new(config.clone())?,
            billing: BillingClient::new(config.clone())?,
            account: AccountClient::new(config.clone())?,
        })
    }
}

pub use account::AccountClient;
pub use agreements::AgreementsClient;
pub use assets::AssetsClient;
pub use audit::AuditClient;
pub use bank::BankClient;
pub use billing::BillingClient;
pub use calendar::CalendarClient;
pub use capture::CaptureClient;
pub use cash::CashClient;
pub use catalog::CatalogClient;
pub use consolidation::ConsolidationClient;
pub use declarations::DeclarationsClient;
pub use document_series::DocumentSeriesClient;
pub use ecommerce::EcommerceClient;
pub use files::FilesClient;
pub use fleet::FleetClient;
pub use hr::HrClient;
pub use inventory::InventoryClient;
pub use leads::LeadsClient;
pub use ledger::LedgerClient;
pub use migration::MigrationClient;
pub use officers::OfficersClient;
pub use operation_types::OperationTypesClient;
pub use partners::PartnersClient;
pub use payroll::PayrollClient;
pub use peppol::PeppolClient;
pub use platform_sellers::PlatformSellersClient;
pub use pos::PosClient;
pub use production::ProductionClient;
pub use projects::ProjectsClient;
pub use public::PublicClient;
pub use purchases::PurchasesClient;
pub use reference::ReferenceClient;
pub use reports::ReportsClient;
pub use sales::SalesClient;
pub use transport::TransportClient;
pub use webhooks::WebhooksClient;

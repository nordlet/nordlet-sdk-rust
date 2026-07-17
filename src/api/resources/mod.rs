//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Reference**
//! - **Partners**
//! - **Catalog**
//! - **Sales**
//! - **Purchases**
//! - **Declarations**
//! - **Ledger**
//! - **Assets**
//! - **Hr**
//! - **Payroll**
//! - **Agreements**
//! - **Inventory**
//! - **Production**
//! - **Ecommerce**
//! - **Cash**
//! - **Transport**
//! - **Pos**
//! - **Audit**
//! - **Webhooks**
//! - **Bank**
//! - **Files**
//! - **Reports**
//! - **Consolidation**
//! - **Public**
//! - **Account**

use crate::{ApiError, ClientConfig};

pub mod account;
pub mod agreements;
pub mod assets;
pub mod audit;
pub mod bank;
pub mod cash;
pub mod catalog;
pub mod consolidation;
pub mod declarations;
pub mod ecommerce;
pub mod files;
pub mod hr;
pub mod inventory;
pub mod ledger;
pub mod partners;
pub mod payroll;
pub mod pos;
pub mod production;
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
    pub catalog: CatalogClient,
    pub sales: SalesClient,
    pub purchases: PurchasesClient,
    pub declarations: DeclarationsClient,
    pub ledger: LedgerClient,
    pub assets: AssetsClient,
    pub hr: HrClient,
    pub payroll: PayrollClient,
    pub agreements: AgreementsClient,
    pub inventory: InventoryClient,
    pub production: ProductionClient,
    pub ecommerce: EcommerceClient,
    pub cash: CashClient,
    pub transport: TransportClient,
    pub pos: PosClient,
    pub audit: AuditClient,
    pub webhooks: WebhooksClient,
    pub bank: BankClient,
    pub files: FilesClient,
    pub reports: ReportsClient,
    pub consolidation: ConsolidationClient,
    pub public: PublicClient,
    pub account: AccountClient,
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            reference: ReferenceClient::new(config.clone())?,
            partners: PartnersClient::new(config.clone())?,
            catalog: CatalogClient::new(config.clone())?,
            sales: SalesClient::new(config.clone())?,
            purchases: PurchasesClient::new(config.clone())?,
            declarations: DeclarationsClient::new(config.clone())?,
            ledger: LedgerClient::new(config.clone())?,
            assets: AssetsClient::new(config.clone())?,
            hr: HrClient::new(config.clone())?,
            payroll: PayrollClient::new(config.clone())?,
            agreements: AgreementsClient::new(config.clone())?,
            inventory: InventoryClient::new(config.clone())?,
            production: ProductionClient::new(config.clone())?,
            ecommerce: EcommerceClient::new(config.clone())?,
            cash: CashClient::new(config.clone())?,
            transport: TransportClient::new(config.clone())?,
            pos: PosClient::new(config.clone())?,
            audit: AuditClient::new(config.clone())?,
            webhooks: WebhooksClient::new(config.clone())?,
            bank: BankClient::new(config.clone())?,
            files: FilesClient::new(config.clone())?,
            reports: ReportsClient::new(config.clone())?,
            consolidation: ConsolidationClient::new(config.clone())?,
            public: PublicClient::new(config.clone())?,
            account: AccountClient::new(config.clone())?,
        })
    }
}

pub use account::AccountClient;
pub use agreements::AgreementsClient;
pub use assets::AssetsClient;
pub use audit::AuditClient;
pub use bank::BankClient;
pub use cash::CashClient;
pub use catalog::CatalogClient;
pub use consolidation::ConsolidationClient;
pub use declarations::DeclarationsClient;
pub use ecommerce::EcommerceClient;
pub use files::FilesClient;
pub use hr::HrClient;
pub use inventory::InventoryClient;
pub use ledger::LedgerClient;
pub use partners::PartnersClient;
pub use payroll::PayrollClient;
pub use pos::PosClient;
pub use production::ProductionClient;
pub use public::PublicClient;
pub use purchases::PurchasesClient;
pub use reference::ReferenceClient;
pub use reports::ReportsClient;
pub use sales::SalesClient;
pub use transport::TransportClient;
pub use webhooks::WebhooksClient;

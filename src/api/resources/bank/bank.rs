use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct BankClient {
    pub http_client: HttpClient,
}

impl BankClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Create a bank, Stripe or broker account. `currency` defaults to the company base currency and `accountCode` to the default bank account of the chart of accounts. A `documentRef` also creates a bank agreement with that document attached. A Stripe account with an `iban` is refused with 422; an account with the same IBAN already existing answers 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn accounts_create(
        &self,
        request: &AccountsCreateBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<AccountsCreateBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/accounts/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the bank accounts of the company. Filterable by name, type, iban and isActive; sortable by name and createdAt.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn accounts_list(
        &self,
        request: &AccountsListBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<AccountsListBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/accounts/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Change the name, type, IBAN, ledger account code or active flag of a bank account. Setting type to stripe clears the IBAN, and sending an `iban` together with it is refused with 422.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn accounts_update(
        &self,
        request: &AccountsUpdateBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<AccountsUpdateBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/accounts/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Insert statement lines (up to 1000 per call) into a bank account as new transactions. A line without `currency` takes the account currency; a line in another currency is refused with 422. A line whose `externalId` is already on the account is skipped and counted in `skipped`. An incoming line that an active match rule (`bank/match-rules/create`) recognises is linked to its settlement batch.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn transactions_import(
        &self,
        request: &TransactionsImportBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<TransactionsImportBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/transactions/import",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Import a statement file into a bank account: camt.053 XML, MT940, a Stripe balance CSV or a Revolut Business CSV, chosen by `format` or by the type of `templateId` (an iso20022 template also takes MT940 content). Statements whose IBAN or currency differ from the account are left out; the call is refused with 422 when none remains, when a line is in another currency, when the file lacks the columns or elements the template requires or when it holds more than 50000 lines. Lines already on the account are skipped, and Stripe movements already posted through a settlement payout are skipped with a warning. With a template, the mapped fields are posted to their ledger accounts, partners are created, payments are matched to the invoices they reference, and invoices, credit notes or advance invoices are issued where the template asks for them (`invoiceMetaField`, `advanceInvoices`); `transfersCsv` posts the lender payouts and commissions of a Stripe template. Invoices issued this way are announced as `sale_invoice.issued`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn statements_import(
        &self,
        request: &StatementsImportBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<StatementsImportBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/statements/import",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the bank transactions of the company. Filterable by id, bankAccountId, status, matchedDocumentId, date, amount, counterpartyIban, counterpartyName and externalId; sortable by date, createdAt and amount.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn transactions_list(
        &self,
        request: &TransactionsListBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<TransactionsListBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/transactions/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Match one new bank transaction to a document: an issued sale invoice or advance for incoming money; an issued sale credit note, a registered purchase invoice or an approved payroll run (company currency only) for outgoing money. Posts the payment journal transaction between the bank account and the receivable, payable or net payable account, with any exchange difference on the gain or loss account, raises the paid amount of the document and sets the line to matched. `invoiceAmount` states the amount settled on an invoice in another currency; in the same currency it must equal the payment (422). Answers 409 when the line is already matched, when the amount exceeds the remaining balance of the document or when the accounting period of the line date is locked. A document that becomes fully paid is announced as `sale_invoice.paid` or `purchase_invoice.paid`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn transactions_match(
        &self,
        request: &TransactionsMatchBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<TransactionsMatchBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/transactions/match",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// The documents are issued sale invoices or advances for incoming money; issued sale credit notes, registered purchase invoices or approved payroll runs for outgoing money. Each must be in the currency of the payment, each allocation must be above zero, a document may appear once and the allocations must add up to the payment (422 otherwise). One journal transaction carries the bank entry, one entry per document and any exchange difference; the line is matched as an `invoice_allocation`. Answers 409 when the line is already matched, when an allocation exceeds the remaining balance of its document or when the accounting period of the line date is locked. Each document that becomes fully paid is announced as `sale_invoice.paid` or `purchase_invoice.paid`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn transactions_match_many(
        &self,
        request: &TransactionsMatchManyBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<TransactionsMatchManyBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/transactions/match-many",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Undo a match. A payment matched to an invoice, or a line posted by an import template, gets a reversing journal transaction dated date (default: today) and the invoice paid amount and payment status are restored; a line linked to a payment-provider settlement is only unlinked. The line returns to status new.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn transactions_unmatch(
        &self,
        request: &TransactionsUnmatchBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<TransactionsUnmatchBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/transactions/unmatch",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Record a payment that is on no imported statement: inserts a bank transaction on the account dated `date` (positive for a sale invoice, negative for a purchase invoice or payroll run, in the account currency) and matches it to the document with the checks, posting and `sale_invoice.paid` or `purchase_invoice.paid` event of `bank/transactions/match`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn transactions_record(
        &self,
        request: &TransactionsRecordBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<TransactionsRecordBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/transactions/record",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Build a SEPA credit transfer file (pain.001.001.03) that pays the remaining amount of each registered purchase invoice from the bank account on `executionDate` (default: today), to the partner's default bank account. The bank account needs an IBAN and every invoice must be in EUR and have a partner bank account on file (422 otherwise, see `partners/bank-accounts/create`); an invoice that is already paid answers 409. The file is returned as `xml`; nothing is posted or marked paid.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn payments_export(
        &self,
        request: &PaymentsExportBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaymentsExportBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/payments/export",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// A template of type stripe, iso20022, revolut or bank_connection names the fields of the file or feed: a field with an `accountCode` is posted to that ledger account on import, a field with `createPartner` creates the customer, supplier or fee partner. `invoiceMetaField` is the Stripe metadata key (stripe) or the reference field (other types) by which payments are linked to invoices; with it `invoiceVatRatePercent`, `invoiceItemId` and, outside Stripe, `advanceInvoices` apply. `companyMetaField`, `lenderMetaField`, the operation types and the refund labels are for Stripe templates only. A template with the same name already existing answers 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn import_templates_create(
        &self,
        request: &ImportTemplatesCreateBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<ImportTemplatesCreateBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/import-templates/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Change a company import template under the rules of `bank/import-templates/create`; a change of `type` re-checks the existing fields against the new type. Built-in templates cannot be changed. A name already used by another template answers 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn import_templates_update(
        &self,
        request: &ImportTemplatesUpdateBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<ImportTemplatesUpdateBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/import-templates/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a company import template. Built-in templates cannot be deleted.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn import_templates_delete(
        &self,
        request: &ImportTemplatesDeleteBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<ImportTemplatesDeleteBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/import-templates/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read one import template, a company one or a built-in one.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn import_templates_get(
        &self,
        request: &ImportTemplatesGetBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<ImportTemplatesGetBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/import-templates/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Today the Revolut Business template: the Revolut CSV fields, with fees, card payments and top-ups posted to the accounts of the posting rules, the sender and beneficiary names creating partners and the Reference column linking payments to invoices. The rows carry `builtin: true`, are readable with `bank/import-templates/get` and are not in `bank/import-templates/list`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn import_templates_list_builtin(
        &self,
        request: &ImportTemplatesListBuiltinBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<ImportTemplatesListBuiltinBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/import-templates/list-builtin",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the import templates of the company (the built-in ones are under `bank/import-templates/list-builtin`). Filterable by name and type; sortable by name and createdAt.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn import_templates_list(
        &self,
        request: &ImportTemplatesListBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<ImportTemplatesListBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/import-templates/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Create a rule that links incoming bank lines to the settlement batches of a payment provider when lines are imported and when a batch is posted. A line whose description, counterparty name or external id contains `pattern` (case-insensitive) is linked to the open batch of `provider` whose payout id follows `payoutIdPrefix` in the line text or, without a prefix, to the single open batch on the same bank account with the same currency and net amount and a payout date within `dateWindowDays` of the line date. `bankAccountId` limits the rule to one account.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn match_rules_create(
        &self,
        request: &MatchRulesCreateBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<MatchRulesCreateBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/match-rules/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Change a match rule; `payoutIdPrefix` and `bankAccountId` are cleared with null.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn match_rules_update(
        &self,
        request: &MatchRulesUpdateBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<MatchRulesUpdateBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/match-rules/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a match rule.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn match_rules_delete(
        &self,
        request: &MatchRulesDeleteBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<MatchRulesDeleteBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/match-rules/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the match rules of the company, ordered by name.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn match_rules_list(
        &self,
        request: &MatchRulesListBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<MatchRulesListBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/match-rules/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Record a SEPA direct debit mandate a partner has signed: debtor IBAN and BIC, scheme (CORE or B2B), recurrent or one-off, and the signature date. `reference` is the mandate reference written into the collection files; without one `MNDT-` followed by a generated id is used. `debtorName` defaults to the partner name. A reference already in use answers 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn mandates_create(
        &self,
        request: &MandatesCreateBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<MandatesCreateBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/mandates/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Change the BIC, debtor name or notes of a mandate.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn mandates_update(
        &self,
        request: &MandatesUpdateBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<MandatesUpdateBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/mandates/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Cancel an active mandate. A mandate that is already cancelled or completed answers 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn mandates_cancel(
        &self,
        request: &MandatesCancelBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<MandatesCancelBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/mandates/cancel",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read one mandate. `expiresOn` is 36 months after the last collection, or after the signature date when nothing has been collected.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn mandates_get(
        &self,
        request: &MandatesGetBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<MandatesGetBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/mandates/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the mandates of the company. Filterable by partnerId, status, scheme, reference and iban; sortable by createdAt and signatureDate.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn mandates_list(
        &self,
        request: &MandatesListBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<MandatesListBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/mandates/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the issued EUR sale invoices that are unpaid or partly paid and whose partner holds an active mandate, each with the mandate `bank/direct-debits/export` would use (the most recently signed active one). Filterable by partnerId, fullNumber, issueDate and dueDate; sortable by issueDate, dueDate, fullNumber, grossTotal and createdAt.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn direct_debits_candidates(
        &self,
        request: &DirectDebitsCandidatesBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<DirectDebitsCandidatesBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/direct-debits/candidates",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Build a SEPA direct debit file (pain.008.001.02) that collects the remaining amount of each issued EUR sale invoice into the bank account on `collectionDate` (default: today) under the partner's most recently signed active mandate. The bank account needs an IBAN and the company a valid `sepaCreditorId` (422, see `account/companies/update`); an invoice whose partner has no active mandate, or whose mandate expired (36 months without a collection), answers 422; an invoice already paid, or a one-off mandate asked to collect more than one invoice, answers 409. The sequence type is OOFF for a one-off mandate, FRST for the first collection under a recurrent one and RCUR after that. On success the collection count and last collection date of each mandate are advanced and a one-off mandate is completed; nothing is posted.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn direct_debits_export(
        &self,
        request: &DirectDebitsExportBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<DirectDebitsExportBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/direct-debits/export",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Suggest documents for a new bank transaction: issued sale invoices and advances for incoming money; registered purchase invoices, issued sale credit notes and approved payroll runs for outgoing money. Candidates are unpaid documents whose remaining balance covers the payment (any balance when in another currency), scored by an equal remaining balance, the document number or payroll period in the payment description and a counterparty IBAN or name matching the partner or an employee on the run; the best `limit` come back with their reasons. A transaction that is already matched answers 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn transactions_suggest_matches(
        &self,
        request: &TransactionsSuggestMatchesBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<TransactionsSuggestMatchesBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/transactions/suggest-matches",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Import a Stripe payout report into settlement batches, one per payout: the itemized payout reconciliation CSV (`format` payout_reconciliation) or the unified payments export with its Transfer column (unified_payments); another CSV is refused with 422. Rows without a payout, the payout rows themselves and unsettled payments are skipped and counted. Each line is matched to an issued sale invoice through the references the report carries (the external reference of a sales order or an invoice number) or through a charge matched earlier. A payout already imported receives its new lines while the batch is in status imported and is skipped once posted; a payout mixing currencies answers 422. Emits `settlement.imported` for a new batch and `settlement.updated` for one that received lines.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn settlements_import(
        &self,
        request: &SettlementsImportBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<SettlementsImportBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/settlements/import",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the settlement batches with their line counts and the clearing figures against the bank lines. Filterable by provider, payoutId, status, currency and bankAccountId; sortable by payoutDate, createdAt and netTotal.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn settlements_list(
        &self,
        request: &SettlementsListBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<SettlementsListBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/settlements/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read one settlement batch with its lines, each with the invoice it is matched to and the bank line that cleared it (`clearingBooked`, `clearingDifference`, `clearingUnposted`).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn settlements_get(
        &self,
        request: &SettlementsGetBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<SettlementsGetBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/settlements/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Match a settlement line to an issued sale invoice or advance by hand (`matchStatus` manual), or clear the match with `invoiceId` null. Only lines of the categories charge, refund, dispute, dispute_reversal and partial_capture_reversal can be matched (422), and only while the batch is in status imported (409).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn settlements_match(
        &self,
        request: &SettlementsMatchBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<SettlementsMatchBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/settlements/match",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// A line with its own rate or amount is split with that value when the batch is posted. A line without one falls back to the commissionPercent given to the posting call, and without that the amount goes to the suspense account. Send both fields as null to clear the line back to the fallback.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn settlements_commission(
        &self,
        request: &SettlementsCommissionBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<SettlementsCommissionBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/settlements/commission",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Attach the incoming bank-statement line that carries this payout to the settlement batch.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn settlements_link(
        &self,
        request: &SettlementsLinkBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<SettlementsLinkBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/settlements/link",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Detach the bank-statement line from the settlement batch and return the line to unmatched.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn settlements_unlink(
        &self,
        request: &SettlementsUnlinkBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<SettlementsUnlinkBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/settlements/unlink",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Post a settlement batch dated `date` (default: the payout date, else today): the bank account receives the net payout, the matched sale invoices are settled against the receivable (the receivable of each partner in a journal transaction of its own when invoices of several partners are settled), the provider fees, the marketplace commission and the seller payable are booked, and a payout in another currency is converted at the rate of that date with the exchange difference against each invoice's own rate on the gain or loss account. A line without a matched invoice is split by its own rate or amount, else by `commissionPercent`, else parked on the suspense account; a line already imported as a movement of a bank account is posted against that account instead. Answers 409 when the batch is already posted or the period is locked, 422 when it has no lines. The payout is linked to the bank line a match rule recognises. Emits `settlement.posted`, `sale_invoice.paid` for each invoice that becomes paid and `refund_liability.actual` for refunds of invoices carrying a refund liability.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn settlements_post(
        &self,
        request: &SettlementsPostBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<SettlementsPostBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/settlements/post",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Import an Interactive Brokers activity statement (CSV) into a bank account of type `broker`; refused with 422 for another account type, a file that is not an IBKR statement, a statement of a different IBKR account than the bank account already holds, or a trade older than the latest trade already imported (statements go in in date order). Rows already imported are skipped. Dividends, withholding tax, interest and fees are posted against the broker account; stock, option and futures trades are matched FIFO against the open lots and post the cost to `securities.investments` and the realised result to `securities.gain` or `securities.loss` (posting rules), one journal entry per date in the base currency at the rate of the day (422 without a rate, 409 when the date's period is locked). Deposits, corporate actions and currency conversions are stored as `not_posted` with a note.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn securities_import(
        &self,
        request: &SecuritiesImportBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<SecuritiesImportBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/securities/import",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the securities imports, newest first; filterable by `bankAccountId` and `brokerAccount`, sortable by `createdAt` and `realizedTotal`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn securities_list(
        &self,
        request: &SecuritiesListBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<SecuritiesListBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/securities/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Return one securities import with its operations in date order, each with its posting status, its amounts in the base currency and the journal transaction it was posted in.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn securities_get(
        &self,
        request: &SecuritiesGetBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<SecuritiesGetBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/securities/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the imported securities operations, newest first; filterable by `bankAccountId`, `kind`, `status`, `symbol` and `date`, sortable by `date` and `createdAt`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn securities_operations_list(
        &self,
        request: &SecuritiesOperationsListBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<SecuritiesOperationsListBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/securities/operations/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read from the bank-feed provider configured on the API server or in platform settings (Enable Banking or an HTTP bridge, named in `provider`), for one `country` or all. Without a provider the call answers 503.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn feeds_banks_list(
        &self,
        request: &FeedsBanksListBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<FeedsBanksListBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/feeds/banks/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Creates a pending bank connection and asks the provider for the authorization page of the bank. `url` is where the user authorizes access for `validForDays` (default 90); `reference` is the state the bank sends back and what `bank/feeds/connections/complete` takes. `redirectUrl` falls back to the one in platform settings, and the call is refused with 422 when neither is set.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn feeds_connections_start(
        &self,
        request: &FeedsConnectionsStartBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<FeedsConnectionsStartBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/feeds/connections/start",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Exchanges `code` for a provider session, sets the connection to active with the consent expiry the bank reports, and records every account of the session as a bank-feed account, not yet linked to a ledger account (`bank/feeds/accounts/link`). A connection that is already active answers 409. When the provider refuses the code the connection goes to status error with the message in `error`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn feeds_connections_complete(
        &self,
        request: &FeedsConnectionsCompleteBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<FeedsConnectionsCompleteBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/feeds/connections/complete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read one bank connection with its bank-feed accounts.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn feeds_connections_get(
        &self,
        request: &FeedsConnectionsGetBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<FeedsConnectionsGetBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/feeds/connections/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the bank connections of the company, without their accounts. Filterable by status, provider and aspspCountry; sortable by createdAt and lastSyncedAt.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn feeds_connections_list(
        &self,
        request: &FeedsConnectionsListBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<FeedsConnectionsListBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/feeds/connections/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Closes the provider session when the connection has one and a provider is configured (a failure at the bank does not stop the call), then deletes the connection and its bank-feed accounts. The ledger bank accounts and the transactions already imported stay.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn feeds_connections_delete(
        &self,
        request: &FeedsConnectionsDeleteBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<FeedsConnectionsDeleteBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/feeds/connections/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Give exactly one of `bankAccountId` (an existing ledger bank account) or `createBankAccount` (a new one with the IBAN and currency of the feed account; `accountCode` defaults to the default bank account of the chart of accounts). `syncFrom` is the date the first sync starts from; without it the first sync covers the last 90 days.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn feeds_accounts_link(
        &self,
        request: &FeedsAccountsLinkBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<FeedsAccountsLinkBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/feeds/accounts/link",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// `importTemplateId` must be a template of type bank_connection, or null to apply none. `syncSchedule` is manual, daily, weekly or monthly; every account whose schedule is not manual is synced by Nordlet when it is due. At least one of the two must be given.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn feeds_accounts_configure(
        &self,
        request: &FeedsAccountsConfigureBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<FeedsAccountsConfigureBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/feeds/accounts/configure",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Fetches the transactions of every linked bank-feed account of the connection, or of `feedAccountId` alone, from `dateFrom` (default: seven days before the last sync, else the `syncFrom` of the account, else 90 days back) to `dateTo`, and inserts the new ones into the linked ledger bank account; lines already there count as `skipped`. An import template set on the account is applied as in `bank/statements/import`. Answers 409 when the connection is not active or has no session, and when the bank returns more than 20 pages for the range (keep `dateFrom` and set an earlier `dateTo`). A provider failure is written to the `error` of the connection, and one that is not retryable sets its status to error. The outcome is announced as `bank_feed.synced`, and invoices the template issues as `sale_invoice.issued`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn feeds_sync(
        &self,
        request: &FeedsSyncBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<FeedsSyncBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/feeds/sync",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}

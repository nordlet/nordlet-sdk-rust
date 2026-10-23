use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct BillingClient {
    pub http_client: HttpClient,
}

impl BillingClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Return the signed-in user's billing account: plan, account type, status, balance, trial and payment state, the month-to-date usage of the companies the user pays for with an estimate of today's charge, the plan catalogue, and the top-up, automatic top-up and overdraft settings.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn account_get(
        &self,
        request: &AccountGetBillingRequest,
        options: Option<RequestOptions>,
    ) -> Result<AccountGetBillingResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/billing/account/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Change the plan of the user's flat-rate billing account; answers 409 when the account type is not `flat_rate`. With a Stripe subscription and payments configured, the subscription is moved to the new plan.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn account_set_plan(
        &self,
        request: &AccountSetPlanBillingRequest,
        options: Option<RequestOptions>,
    ) -> Result<AccountSetPlanBillingResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/billing/account/set-plan",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Switch the user's billing account between `flat_rate` and `per_request`. With a Stripe subscription and payments configured, the subscription is moved to the plan's price for `flat_rate`; for `per_request` the plan is taken off the subscription, and a subscription that carried only the plan is cancelled, leaving the account without one. When Stripe refuses the change, the previous account type is restored.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn account_set_account_type(
        &self,
        request: &AccountSetAccountTypeBillingRequest,
        options: Option<RequestOptions>,
    ) -> Result<AccountSetAccountTypeBillingResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/billing/account/set-account-type",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Switch automatic top-ups on or off and set their amount. Switching them on needs a Stripe customer, which the first card top-up creates (400 otherwise); when switched on, a top-up is charged at once if the balance is at or below zero, no payment has failed and no top-up was requested in the last 24 hours.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn account_set_auto_top_up(
        &self,
        request: &AccountSetAutoTopUpBillingRequest,
        options: Option<RequestOptions>,
    ) -> Result<AccountSetAutoTopUpBillingResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/billing/account/set-auto-top-up",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Set the overdraft cap of the user's billing account: how far usage charges may take the balance below zero before the account is suspended. A suspended account whose balance is within the new cap is activated again.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn account_set_overdraft_cap(
        &self,
        request: &AccountSetOverdraftCapBillingRequest,
        options: Option<RequestOptions>,
    ) -> Result<AccountSetOverdraftCapBillingResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/billing/account/set-overdraft-cap",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Open a Stripe Checkout session for a one-time credit top-up of `amountCents` and return its URL; a Stripe customer is created for the user when there is none. `returnTo` chooses whether Checkout returns to the credits page or to the onboarding plan page, in `locale`. Answers 503 when online payments are not configured.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn topup_create(
        &self,
        request: &TopupCreateBillingRequest,
        options: Option<RequestOptions>,
    ) -> Result<TopupCreateBillingResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/billing/topup/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Confirm a paid top-up Checkout session by `sessionId` and credit the account at once instead of waiting for the Stripe webhook; a session is credited only once. Answers 404 when the session is not the user's own or is not paid.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn topup_confirm(
        &self,
        request: &TopupConfirmBillingRequest,
        options: Option<RequestOptions>,
    ) -> Result<TopupConfirmBillingResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/billing/topup/confirm",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Open a Stripe Checkout session for a flat-rate subscription on `plan`, with the trial period of `trialDays` days, and return its URL; a Stripe customer is created for the user when there is none. Answers 409 when a flat-rate subscription is already active and 503 when online payments are not configured or the plan has no price.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn flat_rate_create(
        &self,
        request: &FlatRateCreateBillingRequest,
        options: Option<RequestOptions>,
    ) -> Result<FlatRateCreateBillingResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/billing/flat-rate/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Confirm a completed flat-rate Checkout session by `sessionId`: the account becomes `flat_rate` on the session's plan with status `active` and the subscription attached, unless the Stripe webhook already did this. Answers 404 when the session is not the user's own or not completed.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn flat_rate_confirm(
        &self,
        request: &FlatRateConfirmBillingRequest,
        options: Option<RequestOptions>,
    ) -> Result<FlatRateConfirmBillingResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/billing/flat-rate/confirm",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Open a Stripe customer portal session that returns to the credits page in `locale`, and return its URL. Answers 503 when online payments are not configured and 400 when the user has no Stripe customer yet.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn portal_create(
        &self,
        request: &PortalCreateBillingRequest,
        options: Option<RequestOptions>,
    ) -> Result<PortalCreateBillingResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/billing/portal/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the newest credit transactions of the user's billing account, up to `limit`: trial grants, top-ups, usage charges, activations and adjustments, each with the balance after it.
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
        request: &TransactionsListBillingRequest,
        options: Option<RequestOptions>,
    ) -> Result<TransactionsListBillingResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/billing/transactions/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the daily usage records of the companies the user pays for, from `from` to `to`: one row per company, date and metric (API requests, OCR pages, file storage bytes, database bytes, sale invoices).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn usage_list(
        &self,
        request: &UsageListBillingRequest,
        options: Option<RequestOptions>,
    ) -> Result<UsageListBillingResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/billing/usage/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}

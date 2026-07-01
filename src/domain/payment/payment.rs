use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{domain::{payment::{payment_method::PaymentMethod, payment_status::PaymentStatus}, shared::money::Money}, provider::payment_provider::PaymentProvider};


#[derive(Debug,Clone)]
pub struct Payment{
    id: Uuid,
    merchant_id: Uuid,
    amount: Money,
    description: Option<String>,
    reference: String,
    status: PaymentStatus,
    payment_method: PaymentMethod,
    provider: PaymentProvider,
    provider_transaction_id: Option<String>,
    failure_reason: Option<String>,
    retry_count: u8,
    idempotency_key: String,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>
}

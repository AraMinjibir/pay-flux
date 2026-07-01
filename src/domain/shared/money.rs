use crate::domain::shared::currency::Currency;


#[derive(Debug, Clone)]
pub struct Money{
    amount:u64,
    currency: Currency
}
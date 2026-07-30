use jiff::Timestamp;

pub mod account;
pub mod errors;
pub mod ledger;
pub mod money;
pub mod transaction;

pub fn uuid_timestamp(timestamp: Timestamp) -> uuid::Timestamp {
    let seconds = u64::try_from(timestamp.as_second()).unwrap_or_default();
    let nanos = u32::try_from(timestamp.subsec_nanosecond()).unwrap_or_default();

    uuid::Timestamp::from_unix(uuid::NoContext, seconds, nanos)
}

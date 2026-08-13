use std::collections::HashMap;

use derive_more::{
    Display,
    From,
    Into,
};
use jiff::Timestamp;
use jiff::civil::{
    Date,
    Time,
};
use zeni_finance::money::Money;
use uuid::Uuid;

use crate::item::{
    ItemId,
    Quantity,
};

pub mod extractors;

#[derive(Debug)]
pub struct Receipt {
    pub header: ReceiptHeader,
    pub articles: Vec<ReceiptArticle>,
}

#[derive(Debug)]
pub struct ReceiptHeader {
    pub id: ReceiptId,
    pub description: String,
    pub created_at: Timestamp,
    pub purchased_at_date: Option<Date>,
    pub purchased_at_time: Option<Time>,
    pub total: Money,
    pub merchant: Option<String>,
    pub additional_fields: HashMap<String, String>,
    pub raw_ocr: String,
}

#[derive(Debug)]
pub struct ReceiptArticle {
    pub item: ItemId,
    pub unit_price: Money,
    pub quantity: Quantity,
    pub total: Money,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Display, From, Into)]
pub struct ReceiptId(Uuid);

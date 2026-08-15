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
use uuid::Uuid;
use zeni_finance::money::Money;

use crate::item::ItemId;

pub mod extractors;

#[derive(Debug, Clone)]
pub struct Receipt {
    pub(crate) id: ReceiptId,
    pub(crate) created_at: Timestamp,
    pub(crate) header: ReceiptHeader,
    pub(crate) articles: Vec<ReceiptArticle>,
}

#[derive(Debug, Clone)]
pub struct ReceiptHeader {
    pub(crate) description: String,
    pub(crate) purchased_at_date: Option<Date>,
    pub(crate) purchased_at_time: Option<Time>,
    pub(crate) total: Money,
    pub(crate) merchant: Option<String>,
    pub(crate) additional_fields: HashMap<String, String>,
    pub(crate) raw_ocr: String,
}

impl ReceiptHeader {
    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn purchased_at_date(&self) -> Option<Date> {
        self.purchased_at_date
    }

    pub fn purchased_at_time(&self) -> Option<Time> {
        self.purchased_at_time
    }

    pub fn total(&self) -> Money {
        self.total
    }

    pub fn merchant(&self) -> Option<&String> {
        self.merchant.as_ref()
    }

    pub fn additional_fields(&self) -> &HashMap<String, String> {
        &self.additional_fields
    }

    pub fn raw_ocr(&self) -> &str {
        &self.raw_ocr
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ReceiptArticle {
    pub(crate) item: ItemId,
    pub(crate) unit_price: Money,
    pub(crate) quantity: Quantity,
    pub(crate) total: Money,
}

impl ReceiptArticle {
    pub fn item(&self) -> ItemId {
        self.item
    }

    pub fn unit_price(&self) -> Money {
        self.unit_price
    }

    pub fn quantity(&self) -> Quantity {
        self.quantity
    }

    pub fn total(&self) -> Money {
        self.total
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quantity {
    Count(u64),
    Weight(u64),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Display, From, Into)]
pub struct ReceiptId(Uuid);

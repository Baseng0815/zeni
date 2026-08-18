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
#[cfg(feature = "extractors")]
use schemars::JsonSchema;
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;
use zeni_finance::money::Money;

use crate::item::ItemId;

pub mod extractors;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    pub(crate) id: ReceiptId,
    pub(crate) created_at: Timestamp,
    pub(crate) header: ReceiptHeader,
    pub(crate) articles: Vec<ReceiptArticle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptHeader {
    pub(crate) purchased_at_date: Option<Date>,
    pub(crate) purchased_at_time: Option<Time>,
    pub(crate) total: Money,
    pub(crate) merchant: Option<Merchant>,
    pub(crate) additional_fields: HashMap<String, String>,
    pub(crate) raw_ocr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "extractors", derive(JsonSchema))]
pub struct Merchant {
    pub(crate) name: String,
    pub(crate) address: String,
}

impl ReceiptHeader {
    pub fn purchased_at_date(&self) -> Option<Date> {
        self.purchased_at_date
    }

    pub fn purchased_at_time(&self) -> Option<Time> {
        self.purchased_at_time
    }

    pub fn total(&self) -> Money {
        self.total
    }

    pub fn merchant(&self) -> Option<&Merchant> {
        self.merchant.as_ref()
    }

    pub fn additional_fields(&self) -> &HashMap<String, String> {
        &self.additional_fields
    }

    pub fn raw_ocr(&self) -> &str {
        &self.raw_ocr
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Quantity {
    Count(u64),
    Weight(u64),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Display, From, Into, Serialize, Deserialize)]
pub struct ReceiptId(Uuid);

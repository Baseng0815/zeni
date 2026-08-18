use std::str::FromStr;

use jiff::Timestamp;
use schemars::JsonSchema;
use serde::Deserialize;
use zeni_finance::money::{
    Currency,
    Money,
};

use crate::errors::{
    InventoryError,
    InventoryResult,
};
use crate::receipt::extractors::{
    ExtractedArticle,
    ExtractedReceipt,
};
use crate::receipt::{
    Merchant,
    Quantity,
    ReceiptHeader,
};

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct RawReceipt {
    pub(crate) merchant: Option<Merchant>,
    pub(crate) purchased_at_date: Option<String>,
    pub(crate) purchased_at_time: Option<String>,
    pub(crate) currency_code: String,
    pub(crate) total: String,
    pub(crate) articles: Vec<RawArticle>,
    pub(crate) additional_fields: Vec<RawField>,
    pub(crate) raw_text: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct RawArticle {
    pub(crate) description: String,
    pub(crate) unit_price: String,
    pub(crate) quantity: RawQuantity,
    pub(crate) total: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct RawQuantity {
    pub(crate) kind: RawQuantityKind,
    pub(crate) count: Option<i64>,
    pub(crate) weight_in_grams: Option<i64>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub(crate) enum RawQuantityKind {
    Count,
    Weight,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct RawField {
    pub(crate) key: String,
    pub(crate) value: String,
}

impl RawReceipt {
    pub(crate) fn into_extraction(self) -> InventoryResult<ExtractedReceipt> {
        let currency = parse_currency(&self.currency_code)?;
        let created_at = Timestamp::now();
        let articles = self
            .articles
            .into_iter()
            .map(|article| article.into_extracted_article(currency))
            .collect::<InventoryResult<Vec<_>>>()?;

        let header = ReceiptHeader {
            purchased_at_date: parse_civil(self.purchased_at_date.as_deref(), "date")?,
            purchased_at_time: parse_civil(self.purchased_at_time.as_deref(), "time")?,
            total: parse_money(&self.total, currency)?,
            merchant: self.merchant,
            additional_fields: self
                .additional_fields
                .into_iter()
                .map(|field| (field.key, field.value))
                .collect(),
            raw_ocr: self.raw_text,
        };

        Ok(ExtractedReceipt { header, articles })
    }
}

impl RawArticle {
    fn into_extracted_article(
        self,
        currency: Currency,
    ) -> InventoryResult<ExtractedArticle> {
        Ok(ExtractedArticle {
            description: self.description,
            unit_price: parse_money(&self.unit_price, currency)?,
            quantity: self.quantity.into_quantity()?,
            total: parse_money(&self.total, currency)?,
        })
    }
}

impl RawQuantity {
    fn into_quantity(self) -> InventoryResult<Quantity> {
        match self.kind {
            RawQuantityKind::Count => self
                .count
                .and_then(|count| u64::try_from(count).ok())
                .map(Quantity::Count)
                .ok_or(InventoryError::MissingQuantity("count")),
            RawQuantityKind::Weight => self
                .weight_in_grams
                .and_then(|grams| u64::try_from(grams).ok())
                .map(Quantity::Weight)
                .ok_or(InventoryError::MissingQuantity("weight")),
        }
    }
}

fn parse_currency(code: &str) -> InventoryResult<Currency> {
    match code.trim().to_ascii_uppercase().as_str() {
        "EUR" | "€" => Ok(Currency::EUR),
        _ => Err(InventoryError::UnsupportedCurrency(code.to_owned())),
    }
}

fn parse_civil<T: FromStr>(
    raw: Option<&str>,
    kind: &'static str,
) -> InventoryResult<Option<T>> {
    raw.map(|value| {
        value.parse().map_err(|_| InventoryError::MalformedCivil {
            kind,
            value: value.to_owned(),
        })
    })
    .transpose()
}

/// Reads a printed amount into minor units, tolerating either separator.
fn parse_money(
    raw: &str,
    currency: Currency,
) -> InventoryResult<Money> {
    let malformed = || InventoryError::MalformedAmount(raw.to_owned());

    let digits: String = raw
        .chars()
        .filter(|character| character.is_ascii_digit() || matches!(character, '.' | ',' | '-'))
        .collect();
    let negative = digits.starts_with('-');
    let digits = digits.trim_start_matches('-');

    let (whole, fraction) = match digits.rfind(['.', ',']) {
        Some(index) if matches!(digits.len() - index - 1, 1 | 2) => (&digits[..index], &digits[index + 1..]),
        _ => (digits, ""),
    };

    let whole = whole.replace(['.', ','], "");
    let whole: i128 = if whole.is_empty() {
        0
    } else {
        whole.parse().map_err(|_| malformed())?
    };
    let fraction: i128 = format!("{fraction:0<2}").parse().map_err(|_| malformed())?;

    let amount = whole
        .checked_mul(100)
        .and_then(|units| units.checked_add(fraction))
        .ok_or_else(malformed)?;

    Ok(Money {
        amount: if negative { -amount } else { amount },
        currency,
    })
}
